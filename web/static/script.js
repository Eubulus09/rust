const courses = [
  {
    id: "algebra", title: "Algebra Basics", subject: "Mathematics", level: "Beginner", icon: "📐", color: "c1",
    blurb: "Build confidence with equations, variables, and the patterns behind them.",
    minutes: 180,
    lessons: [
      { id: "a1", title: "What is a variable?", min: 12, body: "A variable is a symbol — usually a letter — that stands in for a number we don't know yet. It lets us write rules that work for many numbers at once." },
      { id: "a2", title: "Solving simple equations", min: 18, body: "To solve x + 5 = 12, do the same thing to both sides. Subtract 5 from each side: x = 7. Check it by putting 7 back in: 7 + 5 = 12. ✓" },
      { id: "a3", title: "Working with negatives", min: 15, body: "Negative numbers are just positions below zero. Adding a negative moves left; subtracting a negative moves right." },
      { id: "a4", title: "The distributive property", min: 16, body: "a(b + c) = ab + ac. Multiply the outside term by each term inside the brackets, then add." },
      { id: "a5", title: "Inequalities", min: 14, body: "Inequalities work like equations, except multiplying or dividing by a negative number flips the sign." },
      { id: "a6", title: "Graphing a line", min: 20, body: "A straight line follows y = mx + b. m is the slope (rise over run) and b is where the line crosses the y-axis." },
      { id: "a7", title: "Systems of equations", min: 22, body: "Two equations with two unknowns can be solved by substitution or elimination. Both methods aim to remove one variable." },
      { id: "a8", title: "Quadratics intro", min: 24, body: "A quadratic has an x² term and makes a parabola. Factor it, complete the square, or use the quadratic formula." },
      { id: "a9", title: "Practice & review", min: 19, body: "Try mixing the techniques you've learned. Work slowly, show every step, and check each answer by substituting it back." }
    ]
  },
  {
    id: "web", title: "Build Your First Website", subject: "Coding", level: "Beginner", icon: "💻", color: "c2",
    blurb: "HTML, CSS, and a little JavaScript — make something you can share.",
    minutes: 210,
    lessons: [
      { id: "w1", title: "How the web works", min: 14, body: "A browser asks a server for a page. The server sends HTML; the browser reads it and may request CSS, JavaScript, and images." },
      { id: "w2", title: "HTML structure", min: 20, body: "HTML describes content with elements: headings, paragraphs, links, images. Elements can nest inside each other." },
      { id: "w3", title: "Styling with CSS", min: 22, body: "CSS selects elements and applies rules. A rule has a selector and declarations like color: blue; or padding: 10px." },
      { id: "w4", title: "Layout basics", min: 18, body: "Flexbox and Grid make layouts predictable. Flexbox handles one dimension; Grid handles rows and columns." },
      { id: "w5", title: "Making it responsive", min: 16, body: "Use relative units, flexible images, and media queries so your page works on a phone as well as a laptop." },
      { id: "w6", title: "A touch of JavaScript", min: 24, body: "JavaScript reacts to events. Select an element, listen for a click, then change its content or style." },
      { id: "w7", title: "Publish your site", min: 12, body: "Free hosts let you put a static site online in minutes. Your files are the whole deployment." }
    ]
  },
  {
    id: "english", title: "Everyday English", subject: "Languages", level: "All levels", icon: "🗣️", color: "c3",
    blurb: "Practical conversation skills for work, travel, and daily life.",
    minutes: 150,
    lessons: [
      { id: "e1", title: "Introducing yourself", min: 10, body: "Start simple: 'Hi, I'm Ana. I'm from Lima. I work as a nurse.' Short sentences are perfectly fine." },
      { id: "e2", title: "Asking for help", min: 12, body: "'Could you help me, please?' is polite and clear. Add a short reason if you need to." },
      { id: "e3", title: "At the market", min: 14, body: "Useful phrases: 'How much is this?', 'Can I have two, please?', 'Do you have change?'" },
      { id: "e4", title: "Talking about your day", min: 13, body: "Use the past simple for finished actions: 'I went to work. I met a friend. I cooked dinner.'" },
      { id: "e5", title: "Making plans", min: 15, body: "'Let's meet at six.' 'That sounds good.' 'Sorry, I can't — maybe tomorrow?'" },
      { id: "e6", title: "On the phone", min: 16, body: "'Hello, this is Sam speaking.' 'Could you repeat that, please?' 'I'll call back later.'" }
    ]
  },
  {
    id: "biology", title: "Biology: The Human Body", subject: "Science", level: "Beginner", icon: "🧬", color: "c4",
    blurb: "Understand how your body works, from cells to systems.",
    minutes: 165,
    lessons: [
      { id: "b1", title: "Cells: the building blocks", min: 16, body: "Every living thing is made of cells. Cells have a membrane, cytoplasm, and a nucleus holding DNA." },
      { id: "b2", title: "The heart & blood", min: 18, body: "The heart pumps blood around two loops: one to the lungs to pick up oxygen, one to the body to deliver it." },
      { id: "b3", title: "Breathing & oxygen", min: 14, body: "The diaphragm pulls air into the lungs. Oxygen crosses into the blood; carbon dioxide crosses out." },
      { id: "b4", title: "Digestion", min: 17, body: "Food is broken down mechanically and chemically. Nutrients are absorbed in the small intestine." },
      { id: "b5", title: "The nervous system", min: 19, body: "Neurons carry electrical signals. Your brain processes them and sends instructions back to muscles and organs." },
      { id: "b6", title: "Keeping balanced", min: 15, body: "Homeostasis keeps temperature, blood sugar, and water levels in a narrow range so your cells work properly." }
    ]
  },
  {
    id: "money", title: "Money Skills for Life", subject: "Life Skills", level: "All levels", icon: "💰", color: "c5",
    blurb: "Budgeting, saving, and making confident everyday financial choices.",
    minutes: 120,
    lessons: [
      { id: "m1", title: "Where your money goes", min: 14, body: "Track spending for a week. Most people are surprised by the small, frequent purchases that add up." },
      { id: "m2", title: "Building a simple budget", min: 18, body: "Write down income, fixed costs, and flexible costs. Give every remaining unit a job before the month begins." },
      { id: "m3", title: "The habit of saving", min: 12, body: "Save a small amount automatically as soon as you are paid. Consistency matters more than size." },
      { id: "m4", title: "Understanding interest", min: 16, body: "Compound interest grows money over time — but also grows debt. The rate and the time span both matter." },
      { id: "m5", title: "Borrowing wisely", min: 17, body: "Before borrowing, check the total cost, not just the monthly payment. Ask what happens if you miss a payment." }
    ]
  },
  {
    id: "python", title: "Python for Absolute Beginners", subject: "Coding", level: "Beginner", icon: "🐍", color: "c6",
    blurb: "Write your first programs with clear, friendly explanations.",
    minutes: 240,
    lessons: [
      { id: "p1", title: "Your first line of code", min: 12, body: "print('Hello, world!') tells Python to display text. That's a complete program." },
      { id: "p2", title: "Variables & types", min: 16, body: "A variable stores a value. Python figures out the type for you: strings, integers, floats, and booleans." },
      { id: "p3", title: "Making decisions", min: 18, body: "if, elif, and else let your program choose a path based on a condition." },
      { id: "p4", title: "Loops", min: 20, body: "A for loop repeats once for each item in a list. A while loop repeats while a condition is true." },
      { id: "p5", title: "Functions", min: 19, body: "A function packages reusable steps. Give it a name, inputs, and a result." },
      { id: "p6", title: "Lists & dictionaries", min: 22, body: "Lists hold ordered items; dictionaries hold labelled values. You'll use both constantly." },
      { id: "p7", title: "A small project", min: 30, body: "Build a to-do list in the terminal. Combine variables, loops, functions, and lists." }
    ]
  },
  {
    id: "physics", title: "Everyday Physics", subject: "Science", level: "Beginner", icon: "🔭", color: "c7",
    blurb: "The simple rules behind motion, energy, light, and sound.",
    minutes: 140,
    lessons: [
      { id: "ph1", title: "Motion & speed", min: 15, body: "Speed is distance divided by time. Velocity also includes direction." },
      { id: "ph2", title: "Forces", min: 17, body: "A force is a push or pull. When forces don't balance, something accelerates." },
      { id: "ph3", title: "Energy", min: 16, body: "Energy can't be created or destroyed — only changed from one form to another." },
      { id: "ph4", title: "Light & colour", min: 14, body: "White light is a mix of colours. Objects look a certain colour because they reflect that wavelength." },
      { id: "ph5", title: "Sound", min: 13, body: "Sound is a vibration travelling through a medium. No medium, no sound." }
    ]
  },
  {
    id: "health", title: "Health & Wellbeing", subject: "Life Skills", level: "All levels", icon: "🌱", color: "c8",
    blurb: "Practical habits for looking after your body and mind.",
    minutes: 110,
    lessons: [
      { id: "h1", title: "Sleep that restores", min: 14, body: "Consistent timing matters as much as hours. Keep your room cool, dark, and quiet." },
      { id: "h2", title: "Moving your body", min: 15, body: "You don't need a gym. Walking, stairs, and short breaks all count toward being active." },
      { id: "h3", title: "Eating for energy", min: 17, body: "Aim for variety: vegetables, protein, grains, and water. Small steady changes last." },
      { id: "h4", title: "Stress & breathing", min: 13, body: "Slow, deep breathing activates the body's calming system. Try four seconds in, six out." },
      { id: "h5", title: "Asking for support", min: 12, body: "Talking to someone you trust is a strength. If things feel overwhelming, reach out early." }
    ]
  }
];

const subjects = ["All", "Mathematics", "Coding", "Science", "Languages", "Life Skills"];
const languages = ["All", "English", "Spanish", "French", "Hindi", "Arabic", "Turkish", "Russian", "Korean", "German"];

const state = {
  user: null,
  stats: { learners: 0, tutors: 0, courses: 0, answers: 0 },
  tutors: [],
  questions: [],
  progress: {},
  requests: [],
  myTutor: null
};

let courseFilter = "All";
let courseSearch = "";
let tutorFilter = "All";
let tutorSearch = "";

const $ = (selector) => document.querySelector(selector);
const $$ = (selector) => Array.from(document.querySelectorAll(selector));
const esc = (value) => String(value ?? "").replace(/[&<>"']/g, (char) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[char]));

async function api(path, options = {}) {
  const response = await fetch(path, {
    credentials: "same-origin",
    headers: { "Content-Type": "application/json", ...(options.headers || {}) },
    ...options
  });
  const data = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(data.error || "Something went wrong");
  return data;
}

function toast(message) {
  const stack = $("#toast-stack");
  const item = document.createElement("div");
  item.className = "toast";
  item.textContent = message;
  stack.appendChild(item);
  requestAnimationFrame(() => item.classList.add("show"));
  setTimeout(() => {
    item.classList.remove("show");
    setTimeout(() => item.remove(), 300);
  }, 3500);
}

function navigate(view) {
  $$(".view").forEach((section) => section.classList.toggle("active", section.id === `view-${view}`));
  $$(".nav-links a").forEach((link) => link.classList.toggle("active", link.dataset.nav === view));
  $("#nav-links").classList.remove("open");
  $("#menu-toggle").setAttribute("aria-expanded", "false");
  window.scrollTo({ top: 0, behavior: "smooth" });
  if (location.hash !== `#${view}`) history.replaceState(null, "", `#${view}`);
  renderAll();
}

function requireAuth() {
  if (state.user) return true;
  openAuth("register");
  toast("Create a free account to continue.");
  return false;
}

function openModal(html) {
  $("#modal").innerHTML = html;
  $("#modal-backdrop").hidden = false;
  document.body.classList.add("modal-open");
  $("#modal").scrollTop = 0;
}

function closeModal() {
  $("#modal-backdrop").hidden = true;
  document.body.classList.remove("modal-open");
}

function openAuth(mode) {
  const register = mode === "register";
  openModal(`
    <button class="modal-close" data-close aria-label="Close">×</button>
    <span class="modal-eyebrow" style="color:var(--accent)">${register ? "Create your free account" : "Welcome back"}</span>
    <h2 id="modal-title" style="margin:6px 0 8px">${register ? "Start learning today" : "Sign in to OpenClass"}</h2>
    <p class="modal-blurb">${register ? "No fees, no ads, no tracking. Your progress is saved to your account." : "Pick up where you left off."}</p>
    <form class="stack-form" id="auth-form" data-mode="${mode}">
      ${register ? `<label>Your name<input type="text" name="name" maxlength="60" required placeholder="How should we call you?" /></label>` : ""}
      <label>Email<input type="email" name="email" required placeholder="you@example.com" /></label>
      <label>Password<input type="password" name="password" minlength="8" required placeholder="At least 8 characters" /></label>
      <button class="btn btn-primary" type="submit">${register ? "Create account" : "Sign in"}</button>
    </form>
    <button class="link-btn auth-switch" data-auth-switch="${register ? "login" : "register"}" style="margin-top:16px">
      ${register ? "Already have an account? Sign in" : "New here? Create a free account"}
    </button>
  `);
}

function courseProgress(course) {
  const done = state.progress[course.id] || [];
  return { done, total: course.lessons.length, pct: Math.round((done.length / course.lessons.length) * 100) };
}

function courseCard(course) {
  const p = courseProgress(course);
  return `<article class="course-card" data-course="${course.id}">
    <div class="course-visual ${course.color}"><span>${course.icon}</span><small>${esc(course.subject)}</small></div>
    <div class="course-info">
      <div class="course-meta"><span>${esc(course.level)}</span><span>${course.lessons.length} lessons · ${Math.round(course.minutes / 60)}h</span></div>
      <h3>${esc(course.title)}</h3>
      <p>${esc(course.blurb)}</p>
      <div class="progress-track"><span style="width:${p.pct}%"></span></div>
      <div class="course-foot"><span>${state.user ? (p.pct > 0 ? `${p.pct}% complete` : "Not started") : "Sign in to save progress"}</span><strong>${p.pct === 100 ? "Review" : p.pct > 0 ? "Continue" : "Open"} →</strong></div>
    </div>
  </article>`;
}

function renderCourses() {
  const filtered = courses.filter((course) =>
    (courseFilter === "All" || course.subject === courseFilter) &&
    `${course.title} ${course.blurb} ${course.subject}`.toLowerCase().includes(courseSearch.toLowerCase())
  );
  $("#course-grid").innerHTML = filtered.length
    ? filtered.map(courseCard).join("")
    : `<div class="empty-state">No courses match that search. Try a different word or subject.</div>`;
  $("#featured-courses").innerHTML = courses.slice(0, 3).map(courseCard).join("");
}

function renderFilters() {
  $("#subject-filters").innerHTML = subjects.map((subject) =>
    `<button class="chip ${courseFilter === subject ? "selected" : ""}" data-subject="${esc(subject)}">${esc(subject)}</button>`
  ).join("");
  $("#language-filters").innerHTML = languages.map((language) =>
    `<button class="chip ${tutorFilter === language ? "selected" : ""}" data-language="${esc(language)}">${esc(language)}</button>`
  ).join("");
  $("#q-subject").innerHTML = subjects.slice(1).map((subject) => `<option>${esc(subject)}</option>`).join("");
}

function tutorCard(tutor) {
  const initials = tutor.name.split(" ").map((part) => part[0]).join("").slice(0, 2);
  return `<article class="tutor-card">
    <div class="tutor-head"><div class="avatar">${esc(initials)}</div><div><h3>${esc(tutor.name)}</h3><span class="tutor-since">Verified account · volunteer</span></div></div>
    <p>${esc(tutor.bio)}</p>
    <div class="tag-list">${tutor.subjects.map((s) => `<span>${esc(s)}</span>`).join("")}</div>
    <div class="tutor-langs">Speaks: ${tutor.languages.map(esc).join(" · ")} <span class="avail">${esc(tutor.availability || "Flexible")}</span></div>
    <button class="btn btn-ghost tutor-request" data-tutor="${tutor.id}">Request a free session</button>
  </article>`;
}

function renderTutors() {
  if (state.tutors.length === 0) {
    $("#tutor-grid").innerHTML = `<div class="empty-state"><strong>No volunteer tutors have signed up yet.</strong><br />Every profile here would be a real registered account. You can be the first.<br /><button class="link-btn" id="empty-become-tutor" style="margin-top:12px">Become a volunteer tutor →</button></div>`;
    return;
  }
  const filtered = state.tutors.filter((tutor) =>
    (tutorFilter === "All" || tutor.languages.includes(tutorFilter)) &&
    `${tutor.name} ${tutor.bio} ${tutor.subjects.join(" ")} ${tutor.languages.join(" ")}`.toLowerCase().includes(tutorSearch.toLowerCase())
  );
  $("#tutor-grid").innerHTML = filtered.length
    ? filtered.map(tutorCard).join("")
    : `<div class="empty-state">No tutors match those filters right now.</div>`;
}

function questionCard(question) {
  const voted = state.user && question.votes.includes(state.user.id);
  return `<article class="question-card">
    <div class="question-head"><span class="question-subject">${esc(question.subject)}</span><span class="question-author">asked by ${esc(question.author)}</span></div>
    <h3>${esc(question.title)}</h3>
    <p>${esc(question.body)}</p>
    <div class="question-actions">
      <button class="vote-btn ${voted ? "voted" : ""}" data-vote="${esc(question.id)}" aria-label="Upvote">▲ ${question.votes.length}</button>
      <button class="answer-toggle" data-answer="${esc(question.id)}">${question.answers.length ? `${question.answers.length} answer${question.answers.length > 1 ? "s" : ""}` : "Answer"}</button>
    </div>
    <div class="answers" id="answers-${esc(question.id)}" hidden>
      ${question.answers.map((answer) => `<div class="answer"><strong>${esc(answer.author)}</strong><p>${esc(answer.body)}</p></div>`).join("") || `<p class="no-answers">No answers yet. Be the first to help.</p>`}
      <form class="answer-form" data-answer-form="${esc(question.id)}">
        <textarea name="body" placeholder="Share what you know..." rows="2" maxlength="800" required></textarea>
        <button class="btn btn-sm btn-primary" type="submit">Post answer</button>
      </form>
    </div>
  </article>`;
}

function renderQuestions() {
  $("#question-list").innerHTML = state.questions.length
    ? state.questions.map(questionCard).join("")
    : `<div class="empty-state"><strong>No questions yet.</strong><br />Ask the first one — real answers come from real members.</div>`;
}

function renderLearning() {
  if (!state.user) {
    $("#learning-stats").innerHTML = "";
    $("#my-courses").innerHTML = `<div class="empty-state"><strong>Sign in to see your learning.</strong><br /><button class="link-btn" id="empty-signin" style="margin-top:12px">Create a free account →</button></div>`;
    $("#my-requests").innerHTML = `<div class="empty-state">Your tutor requests will appear here once you sign in.</div>`;
    return;
  }
  const started = courses.filter((course) => (state.progress[course.id] || []).length > 0);
  const totalDone = courses.reduce((sum, course) => sum + (state.progress[course.id] || []).length, 0);
  const allLessons = courses.reduce((sum, course) => sum + course.lessons.length, 0);
  const completed = courses.filter((course) => courseProgress(course).pct === 100).length;

  $("#learning-stats").innerHTML = [
    [started.length, "Courses started"],
    [completed, "Courses finished"],
    [totalDone, "Lessons completed"],
    [allLessons ? Math.round((totalDone / allLessons) * 100) + "%" : "0%", "Overall progress"]
  ].map(([value, label]) => `<div class="stat-box"><strong>${value}</strong><span>${label}</span></div>`).join("");

  $("#my-courses").innerHTML = started.length
    ? started.map(courseCard).join("")
    : `<div class="empty-state">You haven't started a course yet. <button class="link-btn" data-go="courses">Browse the catalog →</button></div>`;

  $("#my-requests").innerHTML = state.requests.length
    ? state.requests.map((request) => {
        const tutor = state.tutors.find((t) => t.id === request.tutor_id);
        return `<div class="request-item"><div><strong>${esc(tutor ? tutor.name : "Volunteer tutor")}</strong><p>${esc(request.message)}</p><small>${esc(request.subject)} · ${esc(request.when || "Flexible")} · ${esc(request.email)}</small></div><span class="request-status">Sent</span></div>`;
      }).join("")
    : `<div class="empty-state">No tutor requests yet. <button class="link-btn" data-go="tutors">Find a tutor →</button></div>`;
}

function renderStats() {
  $("#stat-learners").textContent = state.stats.learners;
  $("#stat-tutors").textContent = state.stats.tutors;
  $("#stat-courses").textContent = state.stats.courses;
  $("#stat-answers").textContent = state.stats.answers;
  $("#hero-learners").textContent = state.stats.learners;
  $("#hero-tutors").textContent = state.stats.tutors;
}

function renderAuth() {
  const signedIn = Boolean(state.user);
  $("#user-chip").hidden = !signedIn;
  $("#user-chip").textContent = signedIn ? state.user.name : "";
  $("#auth-btn").hidden = signedIn;
  $("#logout-btn").hidden = !signedIn;
}

function renderAll() {
  renderStats();
  renderAuth();
  renderFilters();
  renderCourses();
  renderTutors();
  renderQuestions();
  renderLearning();
}

function openCourse(courseId) {
  const course = courses.find((item) => item.id === courseId);
  if (!course) return;
  const p = courseProgress(course);
  openModal(`
    <button class="modal-close" data-close aria-label="Close">×</button>
    <div class="modal-head ${course.color}"><span class="modal-icon">${course.icon}</span><div><span class="modal-eyebrow">${esc(course.subject)} · ${esc(course.level)}</span><h2 id="modal-title">${esc(course.title)}</h2></div></div>
    <p class="modal-blurb">${esc(course.blurb)}</p>
    ${state.user ? "" : `<div class="signin-hint">You can read every lesson for free. <button class="link-btn" id="modal-signin">Sign in</button> to save your progress.</div>`}
    <div class="modal-progress"><div class="progress-track"><span style="width:${p.pct}%"></span></div><span>${p.done.length} of ${p.total} lessons complete</span></div>
    <div class="lesson-list">
      ${course.lessons.map((lesson, index) => {
        const done = p.done.includes(lesson.id);
        return `<div class="lesson-row ${done ? "done" : ""}">
          <button class="lesson-check" data-toggle-lesson="${lesson.id}" aria-label="${done ? "Mark incomplete" : "Mark complete"}">${done ? "✓" : index + 1}</button>
          <div class="lesson-text"><strong>${esc(lesson.title)}</strong><p>${esc(lesson.body)}</p></div>
          <span class="lesson-time">${lesson.min} min</span>
        </div>`;
      }).join("")}
    </div>
    <div class="modal-foot"><span>Free forever · No account required to read</span><button class="btn btn-primary" data-close>Done</button></div>
  `);
  $("#modal").dataset.course = courseId;
}

async function toggleLesson(lessonId) {
  if (!requireAuth()) return;
  const courseId = $("#modal").dataset.course;
  const done = (state.progress[courseId] || []).includes(lessonId);
  try {
    await api("/api/progress", {
      method: "POST",
      body: JSON.stringify({ course_id: courseId, lesson_id: lessonId, done: !done })
    });
    const lessons = state.progress[courseId] || [];
    state.progress[courseId] = done ? lessons.filter((id) => id !== lessonId) : [...lessons, lessonId];
    openCourse(courseId);
    renderAll();
  } catch (error) {
    toast(error.message);
  }
}

function openTutorRequest(tutorId) {
  const tutor = state.tutors.find((item) => item.id === tutorId);
  if (!tutor) return;
  openModal(`
    <button class="modal-close" data-close aria-label="Close">×</button>
    <span class="modal-eyebrow" style="color:var(--accent)">Free volunteer session</span>
    <h2 id="modal-title" style="margin:6px 0 8px">Request help from ${esc(tutor.name)}</h2>
    <p class="modal-blurb">${esc(tutor.bio)}</p>
    <form class="stack-form" id="request-form">
      <input type="hidden" name="tutorId" value="${tutor.id}" />
      <label>What do you want help with?<select name="subject" required>${tutor.subjects.map((s) => `<option>${esc(s)}</option>`).join("")}</select></label>
      <label>When are you usually free?<input type="text" name="when" placeholder="e.g. weekday evenings" maxlength="80" required /></label>
      <label>A short message<textarea name="message" rows="4" placeholder="Tell the tutor what you're working on..." maxlength="600" required></textarea></label>
      <button class="btn btn-primary" type="submit">Send request</button>
    </form>
  `);
}

function openBecomeTutor() {
  if (!requireAuth()) return;
  openModal(`
    <button class="modal-close" data-close aria-label="Close">×</button>
    <span class="modal-eyebrow" style="color:var(--accent)">Volunteer</span>
    <h2 id="modal-title" style="margin:6px 0 8px">Become a volunteer tutor</h2>
    <p class="modal-blurb">Your profile will be public and linked to your registered account. List only subjects you can genuinely help with.</p>
    <form class="stack-form" id="tutor-form">
      <label>Subjects you can help with (comma separated)<input type="text" name="subjects" placeholder="Mathematics, Coding" required /></label>
      <label>Languages you speak (comma separated)<input type="text" name="languages" placeholder="English, Spanish" required /></label>
      <label>When are you usually available?<input type="text" name="availability" placeholder="Evenings, weekends" maxlength="80" /></label>
      <label>Short bio<textarea name="bio" rows="3" maxlength="500" required placeholder="Tell learners how you can help..."></textarea></label>
      <button class="btn btn-primary" type="submit">Publish tutor profile</button>
    </form>
  `);
}

document.addEventListener("click", (event) => {
  const goButton = event.target.closest("[data-go]");
  if (goButton) { navigate(goButton.dataset.go); return; }

  const navLink = event.target.closest("[data-nav]");
  if (navLink) { event.preventDefault(); navigate(navLink.dataset.nav); return; }

  const courseCardEl = event.target.closest("[data-course]");
  if (courseCardEl && !event.target.closest("button")) { openCourse(courseCardEl.dataset.course); return; }

  const subjectChip = event.target.closest("[data-subject]");
  if (subjectChip) { courseFilter = subjectChip.dataset.subject; renderCourses(); renderFilters(); return; }

  const languageChip = event.target.closest("[data-language]");
  if (languageChip) { tutorFilter = languageChip.dataset.language; renderTutors(); renderFilters(); return; }

  const lessonToggle = event.target.closest("[data-toggle-lesson]");
  if (lessonToggle) { toggleLesson(lessonToggle.dataset.toggleLesson); return; }

  const tutorRequest = event.target.closest("[data-tutor]");
  if (tutorRequest) { if (requireAuth()) openTutorRequest(tutorRequest.dataset.tutor); return; }

  const vote = event.target.closest("[data-vote]");
  if (vote) {
    if (!requireAuth()) return;
    api(`/api/questions/${vote.dataset.vote}/vote`, { method: "POST" })
      .then(loadAll)
      .catch((error) => toast(error.message));
    return;
  }

  const answerToggle = event.target.closest("[data-answer]");
  if (answerToggle) {
    const panel = document.getElementById(`answers-${answerToggle.dataset.answer}`);
    panel.hidden = !panel.hidden;
    return;
  }

  const authSwitch = event.target.closest("[data-auth-switch]");
  if (authSwitch) { openAuth(authSwitch.dataset.authSwitch); return; }

  if (event.target.closest("#modal-signin") || event.target.closest("#empty-signin")) { openAuth("register"); return; }
  if (event.target.closest("#become-tutor-btn") || event.target.closest("#empty-become-tutor")) { openBecomeTutor(); return; }
  if (event.target.closest("#auth-btn")) { openAuth("register"); return; }

  if (event.target.closest("#logout-btn")) {
    api("/api/logout", { method: "POST" })
      .then(() => { state.user = null; state.progress = {}; state.requests = []; renderAll(); toast("Signed out."); })
      .catch((error) => toast(error.message));
    return;
  }

  if (event.target.closest("#my-learning-btn")) {
    if (!state.user) { openAuth("register"); toast("Sign in to see your learning."); }
    else navigate("learning");
    return;
  }

  if (event.target.closest("[data-close]") || event.target.id === "modal-backdrop") { closeModal(); }
});

document.addEventListener("submit", (event) => {
  const form = event.target;
  event.preventDefault();

  if (form.id === "auth-form") {
    const mode = form.dataset.mode;
    const field = (name) => form.querySelector(`[name="${name}"]`).value.trim();
    const payload = mode === "register"
      ? { name: field("name"), email: field("email"), password: field("password") }
      : { email: field("email"), password: field("password") };
    api(`/api/${mode}`, { method: "POST", body: JSON.stringify(payload) })
      .then(async (data) => {
        state.user = data.user;
        closeModal();
        await loadPrivateData();
        renderAll();
        toast(mode === "register" ? "Account created. Welcome to OpenClass." : "Welcome back.");
      })
      .catch((error) => toast(error.message));
    return;
  }

  if (form.id === "request-form") {
    const field = (name) => form.querySelector(`[name="${name}"]`).value.trim();
    api(`/api/tutors/${field("tutorId")}/request`, {
      method: "POST",
      body: JSON.stringify({ subject: field("subject"), when: field("when"), message: field("message") })
    })
      .then(() => { closeModal(); loadPrivateData(); toast("Request saved. Your tutor can see it in their dashboard."); })
      .catch((error) => toast(error.message));
    return;
  }

  if (form.id === "tutor-form") {
    const field = (name) => form.querySelector(`[name="${name}"]`).value.trim();
    const split = (value) => value.split(",").map((item) => item.trim()).filter(Boolean);
    api("/api/tutors", {
      method: "POST",
      body: JSON.stringify({
        subjects: split(field("subjects")),
        languages: split(field("languages")),
        availability: field("availability"),
        bio: field("bio")
      })
    })
      .then(() => { closeModal(); loadAll(); toast("Your tutor profile is live."); })
      .catch((error) => toast(error.message));
    return;
  }

  if (form.classList.contains("answer-form")) {
    if (!requireAuth()) return;
    api(`/api/questions/${form.dataset.answerForm}/answer`, {
      method: "POST",
      body: JSON.stringify({ body: form.querySelector('[name="body"]').value.trim() })
    })
      .then(() => { loadAll(); toast("Answer posted. Thanks for helping."); })
      .catch((error) => toast(error.message));
    return;
  }

  if (form.id === "ask-form") {
    if (!requireAuth()) return;
    api("/api/questions", {
      method: "POST",
      body: JSON.stringify({
        title: $("#q-title").value.trim(),
        body: $("#q-body").value.trim(),
        subject: $("#q-subject").value
      })
    })
      .then(() => { form.reset(); loadAll(); toast("Your question is live."); })
      .catch((error) => toast(error.message));
  }
});

$("#course-search").addEventListener("input", (event) => { courseSearch = event.target.value; renderCourses(); });
$("#tutor-search").addEventListener("input", (event) => { tutorSearch = event.target.value; renderTutors(); });

$("#menu-toggle").addEventListener("click", () => {
  const open = $("#nav-links").classList.toggle("open");
  $("#menu-toggle").setAttribute("aria-expanded", String(open));
});

$("#theme-toggle").addEventListener("click", () => {
  const next = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
  document.documentElement.dataset.theme = next;
  localStorage.setItem("openclass-theme", next);
  document.querySelector('meta[name="theme-color"]').content = next === "dark" ? "#0a0d12" : "#f5f7f9";
});

document.addEventListener("keydown", (event) => {
  if (event.key === "Escape") closeModal();
});

const onScroll = () => $("#nav").classList.toggle("scrolled", window.scrollY > 20);
onScroll();
window.addEventListener("scroll", onScroll, { passive: true });

const revealObserver = new IntersectionObserver(
  (entries) => {
    entries.forEach((entry) => {
      if (!entry.isIntersecting) return;
      entry.target.classList.add("visible");
      revealObserver.unobserve(entry.target);
    });
  },
  { threshold: 0.1, rootMargin: "0px 0px -40px 0px" }
);
$$(".reveal").forEach((el) => revealObserver.observe(el));

async function loadPrivateData() {
  if (!state.user) return;
  try {
    const [progressData, requests, myTutor] = await Promise.all([
      api("/api/progress"),
      api("/api/requests"),
      api("/api/tutors/mine")
    ]);
    state.progress = Object.fromEntries(progressData.map((entry) => [entry.course_id, entry.lessons]));
    state.requests = requests;
    state.myTutor = myTutor;
  } catch (error) {
    toast(error.message);
  }
}

async function loadAll() {
  try {
    const [me, stats, tutors, questions] = await Promise.all([
      api("/api/me").catch(() => ({ user: null })),
      api("/api/stats"),
      api("/api/tutors"),
      api("/api/questions")
    ]);
    state.user = me.user;
    state.stats = stats;
    state.tutors = tutors;
    state.questions = questions;
    await loadPrivateData();
    renderAll();
  } catch (error) {
    toast(error.message);
  }
}

const savedTheme = localStorage.getItem("openclass-theme");
if (savedTheme) {
  document.documentElement.dataset.theme = savedTheme;
} else if (window.matchMedia("(prefers-color-scheme: light)").matches) {
  document.documentElement.dataset.theme = "light";
}

const initialView = location.hash.replace("#", "");
navigate(["home", "courses", "tutors", "community", "learning"].includes(initialView) ? initialView : "home");
loadAll();
