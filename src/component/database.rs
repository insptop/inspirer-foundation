use crate::Result;
use sea_orm::{Database, DatabaseConnection, DbBackend, Statement, ExecResult, DbErr, QueryResult, DatabaseTransaction};
use std::env;
use std::ops::Deref;

pub use sea_orm::ConnectionTrait;

pub struct Dao<'a, C: ConnectionTrait>(pub &'a C);

#[async_trait]
impl<'a, C: ConnectionTrait> ConnectionTrait for Dao<'a, C> {
    fn get_database_backend(&self) -> DbBackend {
        self.0.get_database_backend()
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        self.0.execute(stmt).await
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        self.0.query_one(stmt).await
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        self.0.query_all(stmt).await
    }
}

pub trait DaoProvider<'a, C: ConnectionTrait> {
    fn dao(&self) -> Dao<'a, C>;
}

impl<'a> DaoProvider<'a, DatabaseConnection> for &'a DatabaseConnection {
    fn dao(&self) -> Dao<'a, DatabaseConnection> {
        Dao(&self)
    }
}

impl<'a> DaoProvider<'a, DatabaseTransaction> for &'a DatabaseTransaction {
    fn dao(&self) -> Dao<'a, DatabaseTransaction> {
        Dao(&self)
    }
}