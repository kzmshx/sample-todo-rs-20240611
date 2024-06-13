use thiserror::Error;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub(super) struct TaskDescription(String);

#[derive(Debug, Error)]
pub(super) enum TaskDescriptionError {
    #[error("Task description must not be empty")]
    Empty,
    #[error("Task description is too long")]
    TooLong,
}

impl TaskDescription {
    pub(super) fn new(value: String) -> Result<Self, TaskDescriptionError> {
        if value.is_empty() {
            return Err(TaskDescriptionError::Empty);
        }
        if value.len() > 2000 {
            return Err(TaskDescriptionError::TooLong);
        }
        Ok(TaskDescription(value))
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
    fn task_description_new() {
        let description = TaskDescription::new("Task description".to_string()).unwrap();
        assert_eq!(description.0, "Task description");
    }
    #[test]
    fn task_description_new_empty() {
        let description = TaskDescription::new("".to_string());
        assert!(description.is_err());
        assert!(description
            .unwrap_err()
            .to_string()
            .contains("Task description must not be empty"));
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
}
