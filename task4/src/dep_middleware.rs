use std::ops::Deref;

use actix_web::web;
use futures::future::LocalBoxFuture;

use crate::dao::{MemoryTaskDao, MemoryTaskListDao, TaskDao, TaskListDao, TaskListMemoryState};

pub struct Dependency<T: ?Sized>(Box<T>);

impl<T: ?Sized> Dependency<T> {
    pub fn new(value: Box<T>) -> Self {
        Self { 0: value }
    }
}

impl<T: ?Sized> Deref for Dependency<T> {
    type Target = Box<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait DependencyFactory<T: ?Sized> {
    fn new(&self, req: &actix_web::HttpRequest) -> LocalBoxFuture<'static, Result<Dependency<T>, actix_web::Error>>;
}

impl DependencyFactory<dyn TaskListDao> for MemoryTaskListDaoFactory {
    fn new(&self, req: &actix_web::HttpRequest) -> LocalBoxFuture<'static, Result<Dependency<dyn TaskListDao>, actix_web::Error>> {
        let state = req.app_data::<TaskListMemoryState>().unwrap().clone();
        Box::pin(async move {
            let value = MemoryTaskListDao::new(state);
            let res: Dependency<dyn TaskListDao> = Dependency::new(Box::new(value));
            Ok(res)
        })
    }
}

impl DependencyFactory<dyn TaskDao> for MemoryTaskDaoFactory {
    fn new(&self, req: &actix_web::HttpRequest) -> LocalBoxFuture<'static, Result<Dependency<dyn TaskDao>, actix_web::Error>> {
        let state = req.app_data::<TaskListMemoryState>().unwrap().clone();
        Box::pin(async move {
            let value = MemoryTaskDao::new(state);
            let res: Dependency<dyn TaskDao> = Dependency::new(Box::new(value));
            Ok(res)
        })
    }
}

pub struct MemoryTaskListDaoFactory;

pub struct MemoryTaskDaoFactory;

impl actix_web::FromRequest for Dependency<dyn TaskListDao + 'static> {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest,
                    _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let factory = req.app_data::<web::Data<dyn DependencyFactory<dyn TaskListDao>>>().unwrap();
        factory.new(req)
    }
}

impl actix_web::FromRequest for Dependency<dyn TaskDao + 'static> {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest,
                    _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let factory = req.app_data::<web::Data<dyn DependencyFactory<dyn TaskDao>>>().unwrap();
        factory.new(req)
    }
}
