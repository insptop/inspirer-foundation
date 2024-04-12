use crate::entity::{apps, domains, users};
use crate::password::password_hash;
use crate::service::app::AppSetting;
use crate::service::init::Init as InitService;
use crate::service::ServiceInterface;
use chrono::Utc;
use clap::Parser;
use inspirer_framework::command::ask;
use inspirer_framework::preludes::*;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;

use crate::app::App;

#[derive(Debug, Parser)]
pub struct InitData;

#[async_trait::async_trait]
impl AppCommand<App> for InitData {
    async fn execute(&self, context: AppContext<App>) -> Result<()> {
        // Truncate old data
        let confirmation = ask("Do you want to truncate old data first?")?;

        if confirmation {
            let backend = context.database.get_database_backend();
            context
                .database
                .execute(backend.build(&Table::truncate().table(domains::Entity).to_owned()))
                .await?;
            println!("[domains] truncated");

            context
                .database
                .execute(backend.build(&Table::truncate().table(apps::Entity).to_owned()))
                .await?;
            println!("[apps] truncated");

            context
                .database
                .execute(backend.build(&Table::truncate().table(users::Entity).to_owned()))
                .await?;
            println!("[users] truncated");
        }

        println!("Ready to init data.");

        let service = context.service::<InitService>();

        // init domain
        let domain_uuid = service.init_domain().await?;

        // init apps
        let app_uuid = service.init_app(domain_uuid).await?;

        // init users
        let user_uuid = service.init_user(domain_uuid).await?;

        println!("Default Domain UUID = {}", domain_uuid);
        println!("Default App UUID = {}", app_uuid);
        println!("Default User UUID = {}", user_uuid);
        println!("Done!");

        Ok(())
    }
}
