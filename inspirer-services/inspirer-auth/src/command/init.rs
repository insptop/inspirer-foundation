use crate::entity::{apps, domains, users};
use crate::password::password_hash;
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

        // init domain
        println!("Initialize domain data.");
        let domain_uuid = Uuid::new_v4();
        domains::Entity::insert(domains::ActiveModel {
            uuid: Set(domain_uuid),
            name: Set("inspirer-auth".into()),
            display_name: Set("InspirerAuthService".into()),
            profile: Set(Some(json!("{}"))),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&context.database)
        .await?;

        // init apps
        println!("Initialize app data.");
        let app_uuid = Uuid::new_v4();
        apps::Entity::insert(apps::ActiveModel {
            uuid: Set(app_uuid),
            domain_uuid: Set(domain_uuid),
            name: Set("inspirer-auth".into()),
            display_name: Set("InspirerAuthService".into()),
            profile: Set(Some(json!("{}"))),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&context.database)
        .await?;

        // init users
        println!("Initialize user data.");
        users::Entity::insert(users::ActiveModel {
            uuid: Set(Uuid::new_v4()),
            domain_uuid: Set(domain_uuid),
            username: Set(Some("inspirer-auth".into())),
            password: Set(password_hash("inspirer-auth")?),
            profile: Set(Some(json!("{}"))),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&context.database)
        .await?;

        println!("Default Domain UUID = {}", domain_uuid);
        println!("Default App UUID = {}", app_uuid);
        println!("Done!");

        Ok(())
    }
}
