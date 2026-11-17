use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::task::{Priority, Status, Task};

#[derive(Debug, Clone, Default)]
pub struct NewTask {
    pub title: String,
    pub notes: Option<String>,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub due: Option<String>,
}

impl NewTask {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            notes: None,
            priority: Priority::Medium,
            tags: Vec::new(),
            due: None,
        }
    }

    fn validate(&self) -> Result<(), AppError> {
        let title = self.title.trim();
        if title.is_empty() {
            return Err(AppError::Validation("title cannot be empty".into()));
        }
        if title.chars().count() > 200 {
            return Err(AppError::Validation(
                "title cannot be longer than 200 characters".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListFilter {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub tag: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct Data {
    #[serde(default)]
    next_id: i64,
    #[serde(default)]
    tasks: Vec<Task>,
}

impl Data {
    fn normalize_next_id(&mut self) {
        let highest = self.tasks.iter().map(|task| task.id).max().unwrap_or(0);
        let next = highest + 1;
        if self.next_id < next {
            self.next_id = next;
        }
    }
}

pub struct Store {
    path: Option<PathBuf>,
    data: Data,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let path = path.as_ref().to_path_buf();
        let mut data = if path.exists() {
            let contents = fs::read_to_string(&path)?;
            if contents.trim().is_empty() {
                Data::default()
            } else {
                serde_json::from_str(&contents)?
            }
        } else {
            Data::default()
        };
        data.normalize_next_id();

        Ok(Self {
            path: Some(path),
            data,
        })
    }

    pub fn in_memory() -> Result<Self, AppError> {
        Ok(Self {
            path: None,
            data: Data::default(),
        })
    }

    fn save(&self) -> Result<(), AppError> {
        if let Some(path) = &self.path {
            let json = serde_json::to_string_pretty(&self.data)?;
            fs::write(path, json)?;
        }
        Ok(())
    }

    pub fn create(&mut self, task: NewTask) -> Result<Task, AppError> {
        task.validate()?;

        let id = self.data.next_id.max(1);
        self.data.next_id = id + 1;

        let created = Task {
            id,
            title: task.title.trim().to_string(),
            notes: clean_optional(task.notes),
            status: Status::Todo,
            priority: task.priority,
            tags: normalize_tags(task.tags),
            due: clean_optional(task.due),
            created_at: Utc::now().to_rfc3339(),
        };

        self.data.tasks.push(created.clone());
        self.save()?;
        Ok(created)
    }

    pub fn get(&self, id: i64) -> Result<Option<Task>, AppError> {
        Ok(self.data.tasks.iter().find(|task| task.id == id).cloned())
    }

    pub fn list(&self, filter: &ListFilter) -> Result<Vec<Task>, AppError> {
        let tag = filter
            .tag
            .as_deref()
            .map(str::trim)
            .filter(|tag| !tag.is_empty());

        let mut tasks: Vec<Task> = self
            .data
            .tasks
            .iter()
            .filter(|task| filter.status.is_none_or(|status| task.status == status))
            .filter(|task| {
                filter
                    .priority
                    .is_none_or(|priority| task.priority == priority)
            })
            .filter(|task| match tag {
                Some(tag) => task.tags.iter().any(|t| t.eq_ignore_ascii_case(tag)),
                None => true,
            })
            .cloned()
            .collect();

        tasks.sort_by_key(|task| (task.is_done(), priority_rank(task.priority), task.id));
        Ok(tasks)
    }

    pub fn set_status(&mut self, id: i64, status: Status) -> Result<(), AppError> {
        {
            let task = self
                .data
                .tasks
                .iter_mut()
                .find(|task| task.id == id)
                .ok_or(AppError::NotFound(id))?;
            task.status = status;
        }
        self.save()
    }

    pub fn update(&mut self, id: i64, task: NewTask) -> Result<Task, AppError> {
        task.validate()?;

        let updated = {
            let existing = self
                .data
                .tasks
                .iter_mut()
                .find(|existing| existing.id == id)
                .ok_or(AppError::NotFound(id))?;

            existing.title = task.title.trim().to_string();
            existing.notes = clean_optional(task.notes);
            existing.priority = task.priority;
            existing.tags = normalize_tags(task.tags);
            existing.due = clean_optional(task.due);
            existing.clone()
        };

        self.save()?;
        Ok(updated)
    }

    pub fn delete(&mut self, id: i64) -> Result<bool, AppError> {
        let before = self.data.tasks.len();
        self.data.tasks.retain(|task| task.id != id);
        let removed = self.data.tasks.len() != before;
        if removed {
            self.save()?;
        }
        Ok(removed)
    }
}

fn clean_optional(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_tags(tags: Vec<String>) -> Vec<String> {
    let mut normalized: Vec<String> = Vec::new();
    for tag in tags {
        let tag = tag.trim();
        if tag.is_empty() {
            continue;
        }
        if !normalized
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(tag))
        {
            normalized.push(tag.to_string());
        }
    }
    normalized
}

fn priority_rank(priority: Priority) -> u8 {
    match priority {
        Priority::High => 0,
        Priority::Medium => 1,
        Priority::Low => 2,
    }
}
