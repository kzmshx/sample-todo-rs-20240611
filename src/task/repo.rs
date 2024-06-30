use super::domain::{ActiveTask, CompletedTask, NewTask, TaskContent, TaskDescription, TaskId};
use chrono::{Local, NaiveDateTime, TimeZone};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("DatabaseError: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, Error)]
pub enum RepoFindError {
    #[error("Task not found")]
    NotFound,
    #[error(transparent)]
    RepoError(#[from] RepoError),
}

pub trait Repo {
    async fn delete_active_task(&self, task: ActiveTask) -> Result<(), RepoError>;

    async fn find_active_task(&self, id: TaskId) -> Result<ActiveTask, RepoFindError>;
    async fn find_active_tasks(&self) -> Result<Vec<ActiveTask>, RepoFindError>;
    async fn find_closed_task(&self, id: TaskId) -> Result<CompletedTask, RepoFindError>;

    async fn save_active_task(&self, task: ActiveTask) -> Result<ActiveTask, RepoError>;
    async fn save_closed_task(&self, task: CompletedTask) -> Result<CompletedTask, RepoError>;
    async fn save_new_task(&self, task: NewTask) -> Result<ActiveTask, RepoError>;
}

pub struct RelationalTaskRepo {
    pool: sqlx::PgPool,
}

#[derive(Debug)]
struct TaskRecord {
    id: i64,
    content: String,
    description: String,
    is_completed: bool,
    created_at: NaiveDateTime,
}

impl From<TaskRecord> for ActiveTask {
    fn from(record: TaskRecord) -> ActiveTask {
        if record.is_completed {
            panic!("TaskRecord is completed");
        }

        ActiveTask::new(
            record.id.into(),
            TaskContent::new(record.content).unwrap(),
            TaskDescription::new(record.description).unwrap(),
            Local.from_local_datetime(&record.created_at).unwrap(),
        )
    }
}

impl From<TaskRecord> for CompletedTask {
    fn from(record: TaskRecord) -> CompletedTask {
        if !record.is_completed {
            panic!("TaskRecord is not completed");
        }

        CompletedTask::new(
            record.id.into(),
            TaskContent::new(record.content).unwrap(),
            TaskDescription::new(record.description).unwrap(),
            Local.from_local_datetime(&record.created_at).unwrap(),
        )
    }
}

impl RelationalTaskRepo {
    pub fn new(pool: sqlx::PgPool) -> Self {
        RelationalTaskRepo { pool }
    }
}

impl Repo for RelationalTaskRepo {
    async fn delete_active_task(&self, task: ActiveTask) -> Result<(), RepoError> {
        let id: i64 = task.id().clone().into();

        sqlx::query!(
            r#"
                DELETE FROM tasks
                WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(RepoError::DatabaseError)?;

        Ok(())
    }

    async fn find_active_task(&self, id: TaskId) -> Result<ActiveTask, RepoFindError> {
        let id: i64 = id.into();
        let record = sqlx::query_as!(
            TaskRecord,
            r#"
                SELECT id, content, description, is_completed, created_at
                FROM tasks
                WHERE id = $1
                  AND is_completed = false
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepoFindError::RepoError(RepoError::DatabaseError(e)))?;

        match record {
            Some(record) => Ok(record.into()),
            None => Err(RepoFindError::NotFound),
        }
    }

    async fn find_active_tasks(&self) -> Result<Vec<ActiveTask>, RepoFindError> {
        let records = sqlx::query_as!(
            TaskRecord,
            r#"
                SELECT id, content, description, is_completed, created_at
                FROM tasks
                WHERE is_completed = false
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepoFindError::RepoError(RepoError::DatabaseError(e)))?;

        Ok(records.into_iter().map(|record| record.into()).collect())
    }

    async fn find_closed_task(&self, id: TaskId) -> Result<CompletedTask, RepoFindError> {
        let id: i64 = id.into();
        let record = sqlx::query_as!(
            TaskRecord,
            r#"
                SELECT id, content, description, is_completed, created_at
                FROM tasks
                WHERE id = $1
                  AND is_completed = true
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepoFindError::RepoError(RepoError::DatabaseError(e)))?;

        match record {
            Some(record) => Ok(record.into()),
            None => Err(RepoFindError::NotFound),
        }
    }

    async fn save_active_task(&self, task: ActiveTask) -> Result<ActiveTask, RepoError> {
        let id: i64 = task.id().clone().into();
        let content: String = task.content().clone().into();
        let description: String = task.description().clone().into();

        let record = sqlx::query_as!(
            TaskRecord,
            r#"
                UPDATE tasks
                SET content = $2,
                    description = $3
                WHERE id = $1
                RETURNING id, content, description, is_completed, created_at
            "#,
            id,
            content,
            description
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepoError::DatabaseError)?;

        Ok(record.into())
    }

    async fn save_closed_task(&self, task: CompletedTask) -> Result<CompletedTask, RepoError> {
        let id: i64 = task.id().clone().into();

        let record = sqlx::query_as!(
            TaskRecord,
            r#"
                UPDATE tasks
                SET is_completed = true
                WHERE id = $1
                RETURNING id, content, description, is_completed, created_at
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepoError::DatabaseError)?;

        Ok(record.into())
    }

    async fn save_new_task(&self, task: NewTask) -> Result<ActiveTask, RepoError> {
        let content: String = task.content().clone().into();
        let description: String = task.description().clone().into();

        let record = sqlx::query_as!(
            TaskRecord,
            r#"
                INSERT INTO tasks (content, description, is_completed, created_at)
                VALUES ($1, $2, $3, $4)
                RETURNING id, content, description, is_completed, created_at
            "#,
            content,
            description,
            false,
            Local::now().naive_local()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepoError::DatabaseError)?;

        Ok(record.into())
    }
}
