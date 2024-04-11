use chrono::Utc;
use clap::Parser;
use inspirer_framework::preludes::*;
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;
use crate::entity::domains;

use crate::app::App;

#[derive(Debug, Parser)]
pub struct InitData;

#[async_trait::async_trait]
impl AppCommand<App> for InitData {
    async fn execute(&self, context: AppContext<App>) -> Result<()> {
        println!("Ready to init data.");

        // init domain
        println!("Initialize domain data.");
        domains::Entity::insert(domains::ActiveModel {
            uuid: Set(Uuid::new_v4()),
            name: Set("inspirer-auth".into()),
            display_name: Set("InspirerAuthService".into()),
            profile: Set(Some(json!("{}"))),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        }).exec(&context.database).await?;

        println!("Done!");

        Ok(())
    }
}