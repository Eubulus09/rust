use tasker_core::{ListFilter, NewTask, Priority, Status, Store};

fn sample(title: &str) -> NewTask {
    NewTask::new(title)
}

#[test]
fn creates_and_lists_tasks() {
    let mut store = Store::in_memory().unwrap();

    let first = store.create(sample("Buy milk")).unwrap();
    let second = store.create(sample("Write report")).unwrap();

    assert_eq!(first.title, "Buy milk");
    assert_eq!(first.status, Status::Todo);
    assert_eq!(second.id, first.id + 1);

    let tasks = store.list(&ListFilter::default()).unwrap();
    assert_eq!(tasks.len(), 2);
}

#[test]
fn rejects_empty_title() {
    let mut store = Store::in_memory().unwrap();
    let err = store.create(sample("   ")).unwrap_err();
    assert!(err.to_string().contains("title cannot be empty"));
}

#[test]
fn filters_by_status_priority_and_tag() {
    let mut store = Store::in_memory().unwrap();

    let mut urgent = NewTask::new("Deploy");
    urgent.priority = Priority::High;
    urgent.tags = vec!["work".into(), "infra".into()];
    let urgent = store.create(urgent).unwrap();

    let mut chore = NewTask::new("Laundry");
    chore.priority = Priority::Low;
    chore.tags = vec!["home".into()];
    store.create(chore).unwrap();

    let high = store
        .list(&ListFilter {
            priority: Some(Priority::High),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(high.len(), 1);
    assert_eq!(high[0].id, urgent.id);

    let work = store
        .list(&ListFilter {
            tag: Some("work".into()),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(work.len(), 1);
    assert_eq!(work[0].title, "Deploy");
}

#[test]
fn toggles_status_and_deletes() {
    let mut store = Store::in_memory().unwrap();
    let task = store.create(sample("Sleep")).unwrap();

    store.set_status(task.id, Status::Done).unwrap();
    let done = store.get(task.id).unwrap().unwrap();
    assert!(done.is_done());

    assert!(store.delete(task.id).unwrap());
    assert!(store.get(task.id).unwrap().is_none());
}

#[test]
fn updates_fields() {
    let mut store = Store::in_memory().unwrap();
    let task = store.create(sample("Old title")).unwrap();

    let mut update = NewTask::new("New title");
    update.notes = Some("with notes".into());
    update.priority = Priority::High;
    update.tags = vec!["a".into(), "b".into()];

    let updated = store.update(task.id, update).unwrap();
    assert_eq!(updated.title, "New title");
    assert_eq!(updated.notes.as_deref(), Some("with notes"));
    assert_eq!(updated.priority, Priority::High);
    assert_eq!(updated.tags, vec!["a".to_string(), "b".to_string()]);
}

#[test]
fn not_found_for_missing_task() {
    let mut store = Store::in_memory().unwrap();
    let err = store.set_status(999, Status::Done).unwrap_err();
    assert!(err.to_string().contains("not found"));
}

#[test]
fn orders_by_status_then_priority() {
    let mut store = Store::in_memory().unwrap();

    let mut low = NewTask::new("Low");
    low.priority = Priority::Low;
    let low = store.create(low).unwrap();

    let mut high = NewTask::new("High");
    high.priority = Priority::High;
    let high = store.create(high).unwrap();

    let mut done = NewTask::new("Done high");
    done.priority = Priority::High;
    let done = store.create(done).unwrap();
    store.set_status(done.id, Status::Done).unwrap();

    let medium = store.create(sample("Medium")).unwrap();

    let ordered: Vec<i64> = store
        .list(&ListFilter::default())
        .unwrap()
        .into_iter()
        .map(|task| task.id)
        .collect();
    assert_eq!(ordered, vec![high.id, medium.id, low.id, done.id]);

    let todo: Vec<i64> = store
        .list(&ListFilter {
            status: Some(Status::Todo),
            ..Default::default()
        })
        .unwrap()
        .into_iter()
        .map(|task| task.id)
        .collect();
    assert_eq!(todo, vec![high.id, medium.id, low.id]);
}

#[test]
fn deduplicates_tags_case_insensitively() {
    let mut store = Store::in_memory().unwrap();

    let mut task = NewTask::new("Errands");
    task.tags = vec![
        "Work".into(),
        " work ".into(),
        "HOME".into(),
        "home".into(),
        "  ".into(),
    ];

    let created = store.create(task).unwrap();
    assert_eq!(created.tags, vec!["Work".to_string(), "HOME".to_string()]);

    let by_tag = store
        .list(&ListFilter {
            tag: Some("work".into()),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(by_tag.len(), 1);
    assert_eq!(by_tag[0].id, created.id);
}

#[test]
fn persists_to_disk_and_reloads() {
    let path = std::env::temp_dir().join(format!("tasker-store-test-{}.json", std::process::id()));
    let _ = std::fs::remove_file(&path);

    let (first_id, second_id) = {
        let mut store = Store::open(&path).unwrap();
        let first = store.create(sample("First")).unwrap();
        let mut second = NewTask::new("Second");
        second.priority = Priority::High;
        second.tags = vec!["persisted".into()];
        let second = store.create(second).unwrap();
        store.set_status(first.id, Status::Done).unwrap();
        (first.id, second.id)
    };

    let reopened = Store::open(&path).unwrap();
    let tasks = reopened.list(&ListFilter::default()).unwrap();
    assert_eq!(tasks.len(), 2);

    let second = reopened.get(second_id).unwrap().unwrap();
    assert_eq!(second.title, "Second");
    assert_eq!(second.priority, Priority::High);
    assert_eq!(second.tags, vec!["persisted".to_string()]);

    let first = reopened.get(first_id).unwrap().unwrap();
    assert!(first.is_done());

    let _ = std::fs::remove_file(&path);
}
