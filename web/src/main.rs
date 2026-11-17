use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use argon2::password_hash::{PasswordHash, SaltString};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::services::ServeDir;
use uuid::Uuid;

const COURSE_COUNT: usize = 8;
const COOKIE_NAME: &str = "openclass_session";

#[derive(Serialize, Deserialize, Clone, Default)]
struct Db {
    users: Vec<User>,
    sessions: Vec<Session>,
    progress: Vec<Progress>,
    tutors: Vec<TutorProfile>,
    requests: Vec<TutorRequest>,
    questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
    password_hash: String,
    created_at: u64,
    is_tutor: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Session {
    token: String,
    user_id: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Progress {
    user_id: String,
    course_id: String,
    lessons: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TutorProfile {
    id: String,
    user_id: String,
    name: String,
    subjects: Vec<String>,
    languages: Vec<String>,
    availability: String,
    bio: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct TutorRequest {
    id: String,
    tutor_id: String,
    user_id: String,
    subject: String,
    #[serde(rename = "when")]
    when_text: String,
    email: String,
    message: String,
    created_at: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct Question {
    id: String,
    user_id: String,
    author: String,
    title: String,
    body: String,
    subject: String,
    created_at: u64,
    votes: Vec<String>,
    answers: Vec<Answer>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Answer {
    id: String,
    user_id: String,
    author: String,
    body: String,
    created_at: u64,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Db>>,
    path: PathBuf,
}

#[derive(Deserialize)]
struct AuthPayload {
    name: Option<String>,
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct ProgressPayload {
    course_id: String,
    lesson_id: String,
    done: bool,
}

#[derive(Deserialize)]
struct TutorPayload {
    subjects: Vec<String>,
    languages: Vec<String>,
    availability: String,
    bio: String,
}

#[derive(Deserialize)]
struct RequestPayload {
    subject: String,
    #[serde(rename = "when")]
    when_text: String,
    message: String,
}

#[derive(Deserialize)]
struct QuestionPayload {
    title: String,
    body: String,
    subject: String,
}

#[derive(Deserialize)]
struct AnswerPayload {
    body: String,
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn load_db(path: &Path) -> Result<Db, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(Db::default());
    }
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn save_db(state: &AppState, db: &Db) -> Result<(), Response> {
    let text = serde_json::to_string_pretty(db)
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Could not save data"))?;
    fs::write(&state.path, text)
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Could not write data file"))?;
    Ok(())
}

fn api_error(status: StatusCode, message: &str) -> Response {
    (status, Json(json!({ "error": message }))).into_response()
}

fn ok(value: Value) -> Response {
    (StatusCode::OK, Json(value)).into_response()
}

fn public_user(user: &User) -> Value {
    json!({ "id": user.id, "name": user.name, "email": user.email, "is_tutor": user.is_tutor })
}

fn session_token(headers: &HeaderMap) -> Option<String> {
    let cookies = headers.get(header::COOKIE)?.to_str().ok()?;
    cookies
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(&format!("{COOKIE_NAME}=")))
        .map(str::to_string)
}

fn current_user(headers: &HeaderMap, db: &Db) -> Option<User> {
    let token = session_token(headers)?;
    let session = db.sessions.iter().find(|session| session.token == token)?;
    db.users.iter().find(|user| user.id == session.user_id).cloned()
}

fn with_session_cookie(mut response: Response, token: &str) -> Response {
    let value = format!("{COOKIE_NAME}={token}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000");
    if let Ok(header_value) = HeaderValue::from_str(&value) {
        response.headers_mut().insert(header::SET_COOKIE, header_value);
    }
    response
}

fn clear_session_cookie(mut response: Response) -> Response {
    let value = format!("{COOKIE_NAME}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0");
    if let Ok(header_value) = HeaderValue::from_str(&value) {
        response.headers_mut().insert(header::SET_COOKIE, header_value);
    }
    response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = PathBuf::from(std::env::var("OPENCLASS_DB").unwrap_or_else(|_| "openclass-data.json".to_string()));
    let db = load_db(&db_path)?;
    let state = AppState { db: Arc::new(Mutex::new(db)), path: db_path };

    let static_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("static");

    let app = Router::new()
        .route("/", get(index))
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/me", get(me))
        .route("/api/stats", get(stats))
        .route("/api/progress", get(get_progress).post(set_progress))
        .route("/api/tutors", get(get_tutors).post(become_tutor))
        .route("/api/tutors/{id}/request", post(request_tutor))
        .route("/api/tutors/mine", get(my_tutor_profile))
        .route("/api/requests", get(my_requests))
        .route("/api/questions", get(get_questions).post(post_question))
        .route("/api/questions/{id}/answer", post(post_answer))
        .route("/api/questions/{id}/vote", post(vote_question))
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state);

    let addr: SocketAddr = std::env::var("TASKER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string())
        .parse()?;

    println!("OpenClass running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Response {
    let name = payload.name.unwrap_or_default().trim().to_string();
    let email = payload.email.trim().to_lowercase();
    let password = payload.password;

    if name.is_empty() || name.len() > 60 {
        return api_error(StatusCode::BAD_REQUEST, "Enter a name between 1 and 60 characters");
    }
    if !email.contains('@') || email.len() > 120 {
        return api_error(StatusCode::BAD_REQUEST, "Enter a valid email address");
    }
    if password.len() < 8 {
        return api_error(StatusCode::BAD_REQUEST, "Password must be at least 8 characters");
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Could not hash password"),
    };

    let user = User {
        id: Uuid::new_v4().to_string(),
        name,
        email: email.clone(),
        password_hash,
        created_at: now(),
        is_tutor: false,
    };
    let token = Uuid::new_v4().to_string();
    let session = Session { token: token.clone(), user_id: user.id.clone(), created_at: now() };

    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    if db.users.iter().any(|existing| existing.email.eq_ignore_ascii_case(&email)) {
        return api_error(StatusCode::CONFLICT, "An account with that email already exists");
    }
    let response_user = public_user(&user);
    db.users.push(user);
    db.sessions.push(session);
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    with_session_cookie(ok(json!({ "user": response_user })), &token)
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> Response {
    let email = payload.email.trim().to_lowercase();
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match db.users.iter().find(|user| user.email.eq_ignore_ascii_case(&email)) {
        Some(user) => user.clone(),
        None => return api_error(StatusCode::UNAUTHORIZED, "Invalid email or password"),
    };
    let parsed_hash = match PasswordHash::new(&user.password_hash) {
        Ok(hash) => hash,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Stored password is invalid"),
    };
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return api_error(StatusCode::UNAUTHORIZED, "Invalid email or password");
    }

    let token = Uuid::new_v4().to_string();
    db.sessions.retain(|session| session.user_id != user.id);
    db.sessions.push(Session { token: token.clone(), user_id: user.id.clone(), created_at: now() });
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    with_session_cookie(ok(json!({ "user": public_user(&user) })), &token)
}

async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    if let Some(token) = session_token(&headers) {
        db.sessions.retain(|session| session.token != token);
        if let Err(response) = save_db(&state, &db) {
            return response;
        }
    }
    clear_session_cookie(ok(json!({ "ok": true })))
}

async fn me(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    match current_user(&headers, &db) {
        Some(user) => ok(json!({ "user": public_user(&user) })),
        None => api_error(StatusCode::UNAUTHORIZED, "Not signed in"),
    }
}

async fn stats(State(state): State<AppState>) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    ok(json!({
        "learners": db.users.len(),
        "tutors": db.tutors.len(),
        "courses": COURSE_COUNT,
        "questions": db.questions.len(),
        "answers": db.questions.iter().map(|question| question.answers.len()).sum::<usize>(),
        "requests": db.requests.len()
    }))
}

async fn get_progress(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to track progress"),
    };
    let entries: Vec<&Progress> = db.progress.iter().filter(|entry| entry.user_id == user.id).collect();
    ok(json!(entries))
}

async fn set_progress(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ProgressPayload>,
) -> Response {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to track progress"),
    };
    if payload.course_id.is_empty() || payload.lesson_id.is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "Invalid lesson");
    }
    let entry = db.progress.iter_mut().find(|entry| entry.user_id == user.id && entry.course_id == payload.course_id);
    match entry {
        Some(entry) => {
            if payload.done && !entry.lessons.contains(&payload.lesson_id) {
                entry.lessons.push(payload.lesson_id.clone());
            } else if !payload.done {
                entry.lessons.retain(|id| id != &payload.lesson_id);
            }
        }
        None => {
            let lessons = if payload.done { vec![payload.lesson_id.clone()] } else { vec![] };
            db.progress.push(Progress { user_id: user.id.clone(), course_id: payload.course_id.clone(), lessons });
        }
    }
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!({ "ok": true }))
}

async fn get_tutors(State(state): State<AppState>) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    ok(json!(db.tutors))
}

async fn my_tutor_profile(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in first"),
    };
    match db.tutors.iter().find(|tutor| tutor.user_id == user.id) {
        Some(tutor) => ok(json!(tutor)),
        None => ok(Value::Null),
    }
}

async fn become_tutor(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<TutorPayload>,
) -> Response {
    if payload.subjects.is_empty() || payload.languages.is_empty() || payload.bio.trim().is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "Add at least one subject, one language, and a short bio");
    }
    if payload.bio.len() > 500 {
        return api_error(StatusCode::BAD_REQUEST, "Bio is too long");
    }
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to become a tutor"),
    };
    let profile = TutorProfile {
        id: Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        name: user.name.clone(),
        subjects: payload.subjects,
        languages: payload.languages,
        availability: payload.availability.trim().to_string(),
        bio: payload.bio.trim().to_string(),
        created_at: now(),
    };
    if let Some(existing) = db.tutors.iter_mut().find(|tutor| tutor.user_id == user.id) {
        *existing = profile.clone();
    } else {
        db.tutors.push(profile.clone());
    }
    if let Some(user_entry) = db.users.iter_mut().find(|entry| entry.id == user.id) {
        user_entry.is_tutor = true;
    }
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!(profile))
}

async fn request_tutor(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(tutor_id): axum::extract::Path<String>,
    Json(payload): Json<RequestPayload>,
) -> Response {
    if payload.subject.trim().is_empty() || payload.message.trim().is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "Add a subject and a short message");
    }
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to request a session"),
    };
    if !db.tutors.iter().any(|tutor| tutor.id == tutor_id) {
        return api_error(StatusCode::NOT_FOUND, "Tutor not found");
    }
    let request = TutorRequest {
        id: Uuid::new_v4().to_string(),
        tutor_id,
        user_id: user.id.clone(),
        subject: payload.subject.trim().to_string(),
        when_text: payload.when_text.trim().to_string(),
        email: user.email.clone(),
        message: payload.message.trim().to_string(),
        created_at: now(),
    };
    db.requests.push(request.clone());
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!(request))
}

async fn my_requests(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in first"),
    };
    let mut requests: Vec<&TutorRequest> = db.requests.iter().filter(|request| request.user_id == user.id).collect();
    requests.sort_by_key(|request| std::cmp::Reverse(request.created_at));
    ok(json!(requests))
}

async fn get_questions(State(state): State<AppState>) -> Response {
    let db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let mut questions = db.questions.clone();
    questions.sort_by_key(|question| std::cmp::Reverse(question.votes.len()));
    ok(json!(questions))
}

async fn post_question(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<QuestionPayload>,
) -> Response {
    if payload.title.trim().is_empty() || payload.body.trim().is_empty() || payload.subject.trim().is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "Fill in the title, subject, and details");
    }
    if payload.title.len() > 140 || payload.body.len() > 1200 {
        return api_error(StatusCode::BAD_REQUEST, "Question is too long");
    }
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to ask a question"),
    };
    let question = Question {
        id: Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        author: user.name.clone(),
        title: payload.title.trim().to_string(),
        body: payload.body.trim().to_string(),
        subject: payload.subject.trim().to_string(),
        created_at: now(),
        votes: vec![],
        answers: vec![],
    };
    db.questions.push(question.clone());
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!(question))
}

async fn post_answer(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(question_id): axum::extract::Path<String>,
    Json(payload): Json<AnswerPayload>,
) -> Response {
    if payload.body.trim().is_empty() {
        return api_error(StatusCode::BAD_REQUEST, "Write an answer first");
    }
    if payload.body.len() > 800 {
        return api_error(StatusCode::BAD_REQUEST, "Answer is too long");
    }
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to answer"),
    };
    let question = match db.questions.iter_mut().find(|question| question.id == question_id) {
        Some(question) => question,
        None => return api_error(StatusCode::NOT_FOUND, "Question not found"),
    };
    let answer = Answer {
        id: Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        author: user.name.clone(),
        body: payload.body.trim().to_string(),
        created_at: now(),
    };
    question.answers.push(answer.clone());
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!(answer))
}

async fn vote_question(
    State(state): State<AppState>,
    headers: HeaderMap,
    axum::extract::Path(question_id): axum::extract::Path<String>,
) -> Response {
    let mut db = match state.db.lock() {
        Ok(db) => db,
        Err(_) => return api_error(StatusCode::INTERNAL_SERVER_ERROR, "Server error"),
    };
    let user = match current_user(&headers, &db) {
        Some(user) => user,
        None => return api_error(StatusCode::UNAUTHORIZED, "Sign in to vote"),
    };
    let question = match db.questions.iter_mut().find(|question| question.id == question_id) {
        Some(question) => question,
        None => return api_error(StatusCode::NOT_FOUND, "Question not found"),
    };
    if question.votes.contains(&user.id) {
        question.votes.retain(|id| id != &user.id);
    } else {
        question.votes.push(user.id.clone());
    }
    let vote_count = question.votes.len();
    if let Err(response) = save_db(&state, &db) {
        return response;
    }
    ok(json!({ "votes": vote_count }))
}
