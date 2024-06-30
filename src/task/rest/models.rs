use serde::{Deserialize, Serialize};

use crate::task::domain::ActiveTask;

#[derive(Debug, Serialize)]
pub(super) struct Task {
    id: i64,
    content: String,
    description: String,
    is_completed: bool,
    created_at: String,
}

impl From<ActiveTask> for Task {
    fn from(task: ActiveTask) -> Self {
        Task {
            id: task.id().clone().into(),
            content: task.content().clone().into(),
            description: task.description().clone().into(),
            is_completed: false,
            created_at: task.created_at().to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct CreateTaskInput {
    content: String,
    description: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct UpdateTaskInput {
    id: i64,
    content: Option<String>,
    description: Option<String>,
}
