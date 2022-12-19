use actix_web::{get, post, Responder, web};
use serde::Deserialize;

use crate::dao::{TaskDao, TaskListDao};
use crate::dep_middleware::Dependency;
use crate::error::TaskListResult;
use crate::model::{task, task_list};
use crate::model::task::TaskIn;
use crate::model::task_list::{TaskListIn, TaskListWithTasks};

use super::view;

#[derive(Deserialize)]
pub struct ListId {
    pub id: task_list::Id,
}

#[derive(Deserialize)]
pub struct TaskId {
    pub list_id: task_list::Id,
    pub id: task::Id,
}

async fn err_or_task_lists(res: TaskListResult<()>,
                           task_list_dao: &Dependency<dyn TaskListDao>) ->
                           TaskListResult<Vec<task_list::TaskListOut>> {
    match res {
        Ok(_) => task_list_dao.get_all().await,
        Err(e) => Err(e)
    }
}

async fn err_or_task_list(res: TaskListResult<()>,
                          list_id: task_list::Id,
                          task_list_dao: &Dependency<dyn TaskListDao>) ->
                          TaskListResult<TaskListWithTasks> {
    match res {
        Ok(_) => task_list_dao.get_by_id(list_id).await,
        Err(e) => Err(e)
    }
}

#[get("/lists")]
pub async fn get_todo_lists(task_list_dao: Dependency<dyn TaskListDao>) -> impl Responder {
    let task_lists = task_list_dao.get_all().await;
    view::view_get_todo_lists(task_lists)
}

#[post("/lists/{id}/drop")]
pub async fn delete_task_list(list_id: web::Path<ListId>,
                              task_list_dao: Dependency<dyn TaskListDao>) -> impl Responder {
    let res = task_list_dao.delete(list_id.id).await;
    view::view_get_todo_lists(err_or_task_lists(res, &task_list_dao).await)
}

#[post("/lists")]
pub async fn add_task_list(task_list: web::Form<TaskListIn>,
                           task_list_dao: Dependency<dyn TaskListDao>) -> impl Responder {
    let res = task_list_dao.add(task_list.into_inner()).await;
    view::view_get_todo_lists(err_or_task_lists(res, &task_list_dao).await)
}

#[get("/lists/{id}")]
pub async fn get_task_list(list_id: web::Path<ListId>,
                           task_list_dao: Dependency<dyn TaskListDao>) -> impl Responder {
    let task_list = task_list_dao.get_by_id(list_id.id).await;
    view::view_get_todo_list(task_list)
}

#[post("/lists/{id}/tasks")]
pub async fn add_task(list_id: web::Path<ListId>,
                      task: web::Form<TaskIn>,
                      task_list_dao: Dependency<dyn TaskListDao>,
                      task_dao: Dependency<dyn TaskDao>) -> impl Responder {
    let res = task_dao
        .add(list_id.id, task.into_inner()).await;
    view::view_get_todo_list(err_or_task_list(res, list_id.id, &task_list_dao).await)
}

#[post("/lists/{list_id}/tasks/{id}/done")]
pub async fn mark_task_as_done(task_id: web::Path<TaskId>,
                               task_list_dao: Dependency<dyn TaskListDao>,
                               task_dao: Dependency<dyn TaskDao>) -> impl Responder {
    let res = task_dao
        .mark_as_done(task_id.list_id, task_id.id).await;
    view::view_get_todo_list(err_or_task_list(res, task_id.list_id, &task_list_dao).await)
}
