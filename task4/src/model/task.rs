use serde::{Deserialize, Serialize};

pub type Id = uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct TaskIn {
    pub name: String,
}

#[derive(Serialize, Clone)]
pub struct TaskOut {
    pub id: Id,
    pub core: TaskIn,
    pub done: bool,
}
