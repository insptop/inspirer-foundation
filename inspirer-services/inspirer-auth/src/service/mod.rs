use std::{marker::PhantomData, ops::Deref};

use inspirer_framework::app::AppContext;

use crate::app::App;

pub mod app;
pub mod init;
pub mod user;

pub struct Service<T> {
    pub(crate) context: AppContext<App>,
    _phantom: PhantomData<T>,
}

pub trait ServiceInterface {
    fn service<T>(&self) -> Service<T>;
}

impl ServiceInterface for AppContext<App> {
    fn service<T>(&self) -> Service<T> {
        Service {
            context: self.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for Service<T> {
    type Target = AppContext<App>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
