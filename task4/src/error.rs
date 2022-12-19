use thiserror::Error;

use crate::model::{task, task_list};

#[derive(Error, Debug)]
pub enum TaskListError {
    #[error("task list for id `{0}` not found")]
    TaskListNotFound(task_list::Id),
    #[error("task for id `{0}` not found")]
    TaskNotFound(task::Id),
    #[error("task {0} already done")]
    TaskAlreadyDone(task::Id),
    // #[error("unexpected server error")]
    // Unknown,
}

pub type TaskListResult<T> = Result<T, TaskListError>;
