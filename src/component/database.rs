use std::ops::Deref;

pub use sea_orm::ConnectionTrait;

use crate::service::Service;

pub struct DaoProject<'a, C> {
    service: Service,
    connection: &'a C,
}

impl<'a, C> Deref for DaoProject<'a, C>
where
    C: ConnectionTrait<'a>,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.connection
    }
}

impl<'a, C> DaoProject<'a, C> {
    pub fn service(&self) -> &Service {
        &self.service
    }
}