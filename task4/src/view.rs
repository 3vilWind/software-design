use actix_web::Responder;
use askama_actix::{Template, TemplateToResponse};
use crate::error::TaskListResult;

use crate::model::task_list;
use crate::model::task_list::TaskListWithTasks;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    text: &'a str,
}

#[derive(Template)]
#[template(path = "lists.html")]
struct ListsTemplate<'a> {
    data: &'a Vec<task_list::TaskListOut>,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate<'a> {
    data: &'a TaskListWithTasks,
}


pub fn view_get_todo_lists(data: TaskListResult<Vec<task_list::TaskListOut>>) -> impl Responder {
    match data {
        Ok(data) => {
            ListsTemplate { data: &data }.to_response()
        }
        Err(err) => {
            ErrorTemplate { text: &*format!("{err:?}") }.to_response()
        }
    }
}

pub fn view_get_todo_list(data: TaskListResult<TaskListWithTasks>) -> impl Responder {
    match data {
        Ok(data) => {
            ListTemplate { data: &data }.to_response()
        }
        Err(err) => {
            ErrorTemplate { text: &*format!("{err:?}") }.to_response()
        }
    }
}