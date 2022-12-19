use std::sync::Arc;

use actix_web::{App, HttpServer};
use actix_web::web::Data;

use crate::dao::{TaskDao, TaskListDao, TaskListMemoryState};
use crate::dep_middleware::{DependencyFactory, MemoryTaskListDaoFactory, MemoryTaskDaoFactory};

mod model;
mod view;
mod controller;
mod dao;
mod dep_middleware;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let memory_state = TaskListMemoryState::new();

    HttpServer::new(move || {
        let task_list_dao: Arc<dyn DependencyFactory<dyn TaskListDao>> =
            Arc::new(MemoryTaskListDaoFactory {});
        let task_dao: Arc<dyn DependencyFactory<dyn TaskDao>> =
            Arc::new(MemoryTaskDaoFactory {});

        App::new()
            .app_data(memory_state.clone())
            .app_data(Data::from(task_list_dao))
            .app_data(Data::from(task_dao))
            .service(controller::get_todo_lists)
            .service(controller::add_task_list)
            .service(controller::delete_task_list)
            .service(controller::get_task_list)
            .service(controller::add_task)
            .service(controller::mark_task_as_done
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
