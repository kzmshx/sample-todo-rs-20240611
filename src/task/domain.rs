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
}
