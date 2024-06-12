use thiserror::Error;

#[derive(Debug, Clone)]
struct TaskId(u64);

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
struct TaskContent(String);

#[derive(Debug, Error)]
enum TaskContentError {
    #[error("Task content must not be empty")]
    Empty,
    #[error("Task content is too long")]
    TooLong,
}

impl TaskContent {
    fn new(content: String) -> Result<Self, TaskContentError> {
        if content.is_empty() {
            return Err(TaskContentError::Empty);
        }
        if content.len() > 500 {
            return Err(TaskContentError::TooLong);
        }
        Ok(TaskContent(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_id_from_u64() {
        let id = TaskId::from(1);
        assert_eq!(id.0, 1);
    }
    #[test]
    fn test_task_id_into_u64() {
        let id = TaskId(1);
        let val: u64 = id.into();
        assert_eq!(val, 1);
    }

    #[test]
    fn task_content_new() {
        let content = TaskContent::new("Hello".to_string()).unwrap();
        assert_eq!(content.0, "Hello");
    }
    #[test]
    fn task_content_new_empty() {
        let content = TaskContent::new("".to_string());
        assert!(content.is_err());
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
}
