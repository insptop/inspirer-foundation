use clap::Parser;
use inspirer_framework::{app::AppContext, command::AppCommand, Result};
use sea_orm::EntityTrait;

use crate::{app::App, entity};

/// Test command
#[derive(Parser, Debug)]
pub struct Test {
    echo: String
}

#[async_trait::async_trait]
impl AppCommand<App> for Test {
    async fn execute(&self, context: AppContext<App>) -> Result<()> {
        println!("begin: {}", self.echo);
        let users = entity::users::Entity::find().all(&context.database).await?;
        println!("end: {}", self.echo);
        println!("{:#?}", users);

        Ok(())
    }
}