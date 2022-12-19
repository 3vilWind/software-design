use serde::{Serialize, Deserialize};
use super::task::TaskOut;

pub type Id = uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskListIn {
    pub name: String,
}

#[derive(Serialize, Clone)]
pub struct TaskListOut {
    pub id: Id,
    pub core: TaskListIn,
}

#[derive(Serialize, Clone)]
pub struct TaskListWithTasks {
    pub id: Id,
    pub core: TaskListIn,
    pub tasks: Vec<TaskOut>,
}
