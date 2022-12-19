use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::error::{TaskListError, TaskListResult};
use crate::model::task::{TaskIn, TaskOut};
use crate::model::task_list::{TaskListIn, TaskListOut, TaskListWithTasks};

use super::model::{task, task_list};

#[async_trait]
pub trait TaskListDao {
    async fn add(&self, data: TaskListIn) -> TaskListResult<()>;
    async fn delete(&self, id: task_list::Id) -> TaskListResult<()>;
    async fn get_all(&self) -> TaskListResult<Vec<TaskListOut>>;
    async fn get_by_id(&self, id: task_list::Id) -> TaskListResult<TaskListWithTasks>;
}

#[async_trait]
pub trait TaskDao {
    async fn add(&self, task_list_id: task_list::Id, data: TaskIn) -> TaskListResult<()>;
    async fn mark_as_done(&self, task_list_id: task_list::Id, id: task::Id) -> TaskListResult<()>;
}

#[derive(Clone)]
pub struct TaskListMemoryState(Arc<Mutex<HashMap<task_list::Id, TaskListWithTasks>>>);

impl TaskListMemoryState {
    pub fn new() -> Self {
        TaskListMemoryState { 0: Arc::new(Mutex::new(HashMap::new())) }
    }
}

impl Deref for TaskListMemoryState {
    type Target = Arc<Mutex<HashMap<task_list::Id, TaskListWithTasks>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct MemoryTaskListDao {
    state: TaskListMemoryState,
}

impl MemoryTaskListDao {
    pub fn new(state: TaskListMemoryState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl TaskListDao for MemoryTaskListDao {
    async fn add(&self, data: TaskListIn) -> TaskListResult<()> {
        let mut state = self.state.lock().await;
        let next_id = Uuid::new_v4();
        state.insert(next_id,
                     TaskListWithTasks { id: next_id, core: TaskListIn { name: data.name }, tasks: vec![] });
        Ok(())
    }

    async fn delete(&self, id: task_list::Id) -> TaskListResult<()> {
        self.state
            .lock()
            .await
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| TaskListError::TaskListNotFound(id))
    }

    async fn get_all(&self) -> TaskListResult<Vec<TaskListOut>> {
        Ok(self.state
            .lock()
            .await
            .iter()
            .map(|(_, x)|
                TaskListOut { id: x.id, core: TaskListIn { name: x.core.name.clone() } }
            )
            .collect())
    }

    async fn get_by_id(&self, id: task_list::Id) -> TaskListResult<TaskListWithTasks> {
        self.state
            .lock()
            .await
            .get(&id)
            .map(|x| x.clone())
            .ok_or_else(|| TaskListError::TaskListNotFound(id))
    }
}

pub struct MemoryTaskDao {
    state: TaskListMemoryState,
}

impl MemoryTaskDao {
    pub fn new(state: TaskListMemoryState) -> Self {
        Self { state }
    }
}

#[async_trait]
impl TaskDao for MemoryTaskDao {
    async fn add(&self, task_list_id: task_list::Id, data: TaskIn) -> TaskListResult<()> {
        self.state
            .lock()
            .await
            .get_mut(&task_list_id)
            .map(|task|
                task.tasks.push(TaskOut { core: data, id: Uuid::new_v4(), done: false }))
            .ok_or_else(|| TaskListError::TaskListNotFound(task_list_id))
    }

    async fn mark_as_done(&self, task_list_id: task_list::Id, id: task::Id) -> TaskListResult<()> {
        self.state
            .lock()
            .await
            .get_mut(&task_list_id)
            .map(|task_list| {
                task_list.tasks
                    .iter_mut()
                    .find(|x| x.id == id)
                    .map(|task| if !task.done {
                        task.done = true;
                        Ok(())
                    } else {
                        Err(TaskListError::TaskAlreadyDone(id))
                    })
                    .unwrap_or(Err(TaskListError::TaskNotFound(id)))
            })
            .unwrap_or(Err(TaskListError::TaskListNotFound(task_list_id)))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_lists() {
        let memory_state = TaskListMemoryState::new();
        let task_lists_dao = MemoryTaskListDao::new(memory_state.clone());

        let res = task_lists_dao.add(TaskListIn { name: "hello".to_owned() }).await;
        assert!(res.is_ok());

        let task_lists = task_lists_dao.get_all().await.unwrap();
        assert_eq!(task_lists.len(), 1);
        let task = task_lists.get(0).unwrap();
        assert_eq!(task.core.name, "hello");
        let res = task_lists_dao.delete(task.id).await;
        assert!(res.is_ok());
        let task_lists = task_lists_dao.get_all().await.unwrap();
        assert_eq!(task_lists.len(), 0);
    }

    #[actix_web::test]
    async fn test_list() {
        let memory_state = TaskListMemoryState::new();
        let task_lists_dao = MemoryTaskListDao::new(memory_state.clone());
        let task_dao = MemoryTaskDao::new(memory_state.clone());

        task_lists_dao.add(TaskListIn { name: "hello".to_owned() }).await.unwrap();
        let task_lists = task_lists_dao.get_all().await.unwrap();
        let task_list = task_lists.get(0).unwrap();

        let res = task_dao.add(task_list.id, TaskIn { name: "test".to_owned() }).await;
        assert!(res.is_ok());

        let res = task_lists_dao.get_by_id(task_list.id).await;
        assert!(res.is_ok());
        let task_list = res.unwrap();
        assert_eq!(task_list.tasks.len(), 1);
        let task = task_list.tasks.get(0).unwrap();
        assert_eq!(task.core.name, "test");
        assert_eq!(task.done, false);

        let res = task_dao.mark_as_done(task_list.id, task.id).await;
        assert!(res.is_ok());
        let task_list = task_lists_dao.get_by_id(task_list.id).await.unwrap();
        let task = task_list.tasks.get(0).unwrap();
        assert_eq!(task.done, true);
    }
}
