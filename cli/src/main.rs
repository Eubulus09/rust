use std::path::PathBuf;
use std::process::ExitCode;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use tasker_core::{AppError, ListFilter, NewTask, Priority, Status, Store, Task};

#[derive(Parser)]
#[command(name = "tasker", version, about = "A tiny, fast task manager")]
struct Cli {
    /// Path to the data file.
    #[arg(long, global = true, env = "TASKER_DB", default_value = "tasks.json")]
    db: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new task.
    Add {
        title: String,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long, default_value = "medium")]
        priority: String,
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long)]
        due: Option<String>,
    },
    /// List tasks, optionally filtered.
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        tag: Option<String>,
    },
    /// Mark a task as done.
    Done { id: i64 },
    /// Reopen a completed task.
    Undone { id: i64 },
    /// Edit an existing task.
    Edit {
        id: i64,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        notes: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long = "tag")]
        tags: Vec<String>,
        #[arg(long)]
        due: Option<String>,
    },
    /// Delete a task.
    Delete { id: i64 },
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let mut store = Store::open(&cli.db)?;

    match cli.command {
        Command::Add {
            title,
            notes,
            priority,
            tags,
            due,
        } => {
            let mut task = NewTask::new(title);
            task.notes = notes;
            task.priority = Priority::from_str(&priority)?;
            task.tags = tags;
            task.due = due;
            let task = store.create(task)?;
            println!("Added task #{}: {}", task.id, task.title);
        }
        Command::List {
            status,
            priority,
            tag,
        } => {
            let filter = ListFilter {
                status: status.map(|value| Status::from_str(&value)).transpose()?,
                priority: priority
                    .map(|value| Priority::from_str(&value))
                    .transpose()?,
                tag,
            };
            let tasks = store.list(&filter)?;
            print_tasks(&tasks);
        }
        Command::Done { id } => {
            store.set_status(id, Status::Done)?;
            println!("Completed task #{id}");
        }
        Command::Undone { id } => {
            store.set_status(id, Status::Todo)?;
            println!("Reopened task #{id}");
        }
        Command::Edit {
            id,
            title,
            notes,
            priority,
            tags,
            due,
        } => {
            let existing = store.get(id)?.ok_or(AppError::NotFound(id))?;

            let mut update = NewTask::new(title.unwrap_or(existing.title));
            update.notes = notes.or(existing.notes);
            update.priority = match priority {
                Some(value) => Priority::from_str(&value)?,
                None => existing.priority,
            };
            update.tags = if tags.is_empty() { existing.tags } else { tags };
            update.due = due.or(existing.due);

            let updated = store.update(id, update)?;
            println!("Updated task #{}: {}", updated.id, updated.title);
        }
        Command::Delete { id } => {
            if store.delete(id)? {
                println!("Deleted task #{id}");
            } else {
                return Err(AppError::NotFound(id));
            }
        }
    }

    Ok(())
}

fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks found.");
        return;
    }

    for task in tasks {
        let checkbox = if task.is_done() { "x" } else { " " };
        println!(
            "[{checkbox}] #{:<3} {:<6} {:<6} {}",
            task.id, task.priority, task.status, task.title
        );

        if let Some(due) = &task.due {
            println!("          due: {due}");
        }
        if let Some(notes) = &task.notes {
            println!("          notes: {notes}");
        }
        if !task.tags.is_empty() {
            println!("          tags: {}", task.tags.join(", "));
        }
    }
}
