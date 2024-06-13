use chrono::{DateTime, Local};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TaskId(u64);

impl From<u64> for TaskId {
    fn from(id: u64) -> Self {
        TaskId(id)
    }
}

impl From<TaskId> for u64 {
    fn from(val: TaskId) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TaskContent(String);

#[derive(Debug, Error)]
pub(super) enum TaskContentError {
    #[error("Task content must not be empty")]
    Empty,
    #[error("Task content is too long")]
    TooLong,
}

impl TaskContent {
    pub(super) fn new(value: String) -> Result<Self, TaskContentError> {
        if value.is_empty() {
            return Err(TaskContentError::Empty);
        }
        if value.len() > 500 {
            return Err(TaskContentError::TooLong);
        }
        Ok(TaskContent(value))
    }

    fn new_or_panic(value: String) -> Self {
        TaskContent::new(value).unwrap()
    }
}

impl From<TaskContent> for String {
    fn from(val: TaskContent) -> Self {
        val.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct TaskDescription(String);

#[derive(Debug, Error)]
pub(super) enum TaskDescriptionError {
    #[error("Task description is too long")]
    TooLong,
}

impl TaskDescription {
    pub(super) fn new(value: String) -> Result<Self, TaskDescriptionError> {
        if value.len() > 2000 {
            return Err(TaskDescriptionError::TooLong);
        }
        Ok(TaskDescription(value))
    }

    fn new_or_panic(value: String) -> Self {
        TaskDescription::new(value).unwrap()
    }
}

impl From<TaskDescription> for String {
    fn from(val: TaskDescription) -> Self {
        val.0.clone()
    }
}

#[derive(Debug)]
pub(super) struct NewTask {
    content: TaskContent,
    description: TaskDescription,
}

impl NewTask {
    pub(super) fn new(content: TaskContent, description: TaskDescription) -> Self {
        NewTask {
            content,
            description,
        }
    }
}

#[derive(Debug)]
pub(super) struct ActiveTask {
    id: TaskId,
    content: TaskContent,
    description: TaskDescription,
    created_at: DateTime<Local>,
}

impl ActiveTask {
    pub(super) fn modify_content(self, content: TaskContent) -> Self {
        Self {
            id: self.id,
            content,
            description: self.description,
            created_at: self.created_at,
        }
    }

    pub(super) fn modify_description(self, description: TaskDescription) -> Self {
        Self {
            id: self.id,
            content: self.content,
            description,
            created_at: self.created_at,
        }
    }

    pub(super) fn close(self) -> CompletedTask {
        CompletedTask {
            id: self.id,
            content: self.content,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug)]
pub(super) struct CompletedTask {
    id: TaskId,
    content: TaskContent,
    description: TaskDescription,
    created_at: DateTime<Local>,
}

impl CompletedTask {
    pub(super) fn reopen(self) -> ActiveTask {
        ActiveTask {
            id: self.id,
            content: self.content,
            description: self.description,
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_id_from_u64() {
        let id = TaskId::from(1);
        assert_eq!(id.0, 1);
    }
    #[test]
    fn task_id_into_u64() {
        let id = TaskId(1);
        let val: u64 = id.into();
        assert_eq!(val, 1);
    }

    #[test]
    fn task_content_new() {
        let content = TaskContent::new("Task content".to_string()).unwrap();
        assert_eq!(content.0, "Task content");
    }
    #[test]
    fn task_content_new_empty() {
        let content = TaskContent::new("".to_string());
        assert!(content.is_err());
        assert!(content
            .unwrap_err()
            .to_string()
            .contains("Task content must not be empty"));
    }
    #[test]
    fn task_content_new_too_long() {
        let content = TaskContent::new("a".repeat(501));
        assert!(content.is_err());
        assert!(content
            .unwrap_err()
            .to_string()
            .contains("Task content is too long"));
    }
    #[test]
    fn task_content_into_string() {
        let content = TaskContent::new("Task content".to_string()).unwrap();
        let val: String = content.into();
        assert_eq!(val, "Task content");
    }

    #[test]
    fn task_description_new() {
        let description = TaskDescription::new("Task description".to_string()).unwrap();
        assert_eq!(description.0, "Task description");
    }
    #[test]
    fn task_description_new_too_long() {
        let description = TaskDescription::new("a".repeat(2001));
        assert!(description.is_err());
        assert!(description
            .unwrap_err()
            .to_string()
            .contains("Task description is too long"));
    }
    #[test]
    fn task_description_into_string() {
        let description = TaskDescription::new("Task description".to_string()).unwrap();
        let val: String = description.into();
        assert_eq!(val, "Task description");
    }

    #[test]
    fn task_operations() {
        let task = ActiveTask {
            id: TaskId(1),
            content: TaskContent::new_or_panic("Task content".into()),
            description: TaskDescription::new_or_panic("Task description".into()),
            created_at: Local::now(),
        };
        assert_eq!(task.id, TaskId(1));
        assert_eq!(
            task.content,
            TaskContent::new_or_panic("Task content".into())
        );
        assert_eq!(
            task.description,
            TaskDescription::new_or_panic("Task description".into())
        );

        let task = task.modify_content(TaskContent::new_or_panic("New content".into()));
        assert_eq!(
            task.content,
            TaskContent::new_or_panic("New content".into())
        );

        let task = task.modify_description(TaskDescription::new_or_panic("New description".into()));
        assert_eq!(
            task.description,
            TaskDescription::new_or_panic("New description".into())
        );

        task.close().reopen();
    }
}
