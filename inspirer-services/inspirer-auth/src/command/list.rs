use clap::{Parser, ValueEnum};
use inspirer_framework::preludes::*;
use sea_orm::EntityTrait;
use tabled::Table;

use crate::{
    app::App,
    entity::{apps, domains, users},
};

#[derive(Parser)]
pub struct List {
    #[arg(value_enum, value_name = "TARGET")]
    target: ListData,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ListData {
    Application,
    Domain,
    User,
}

#[async_trait::async_trait]
impl AppCommand<App> for List {
    async fn execute(&self, context: AppContext<App>) -> Result<()> {
        let table = match self.target {
            ListData::Application => {
                let apps = apps::Entity::find().all(&context.database).await?;
                Table::new(&apps).to_string()
            }
            ListData::Domain => {
                let domains = domains::Entity::find().all(&context.database).await?;
                Table::new(&domains).to_string()
            }
            ListData::User => {
                let users = users::Entity::find().all(&context.database).await?;
                Table::new(&users).to_string()
            }
        };

        println!("{}", table);

        Ok(())
    }
}
