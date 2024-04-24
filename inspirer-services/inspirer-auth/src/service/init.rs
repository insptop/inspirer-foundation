use crate::{
    auth::{application::AppSetting, user::Gender},
    config::AppConfig,
    entity::{apps, domains, users},
    password::password_hash,
};

use super::Service;
use chrono::Utc;
use inspirer_framework::preludes::*;
use openidconnect::{StandardClaims, SubjectIdentifier};
use rand::{rngs::OsRng, RngCore};
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;

pub struct Init;

impl Service<Init> {
    pub async fn init_domain(&self) -> Result<Uuid> {
        let config = self.config.get::<AppConfig>("app")?;

        println!("Initialize domain data.");
        let domain_uuid = Uuid::new_v4();
        domains::Entity::insert(domains::ActiveModel {
            uuid: Set(domain_uuid),
            name: Set(config.app_name.clone()),
            display_name: Set(config.app_name.clone()),
            profile: Set(json!("{}")),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&self.database)
        .await?;

        Ok(domain_uuid)
    }

    pub async fn init_app(&self, domain_uuid: Uuid) -> Result<Uuid> {
        let config = self.config.get::<AppConfig>("app")?;

        let mut key = [0u8; 16];
        OsRng.fill_bytes(&mut key);

        println!("Initialize app data.");
        let app_uuid = Uuid::new_v4();
        apps::Entity::insert(apps::ActiveModel {
            uuid: Set(app_uuid),
            domain_uuid: Set(domain_uuid),
            name: Set(config.app_name.clone()),
            display_name: Set(config.app_name.clone()),
            secret: Set(key.to_vec()),
            profile: Set(json!("{}")),
            setting: Set(AppSetting::default()),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&self.database)
        .await?;

        Ok(app_uuid)
    }

    pub async fn init_user(&self, domain_uuid: Uuid) -> Result<Uuid> {
        let config = self.config.get::<AppConfig>("app")?;

        println!("Initialize user data.");
        let user_uuid = Uuid::new_v4();
        users::Entity::insert(users::ActiveModel {
            uuid: Set(user_uuid),
            domain_uuid: Set(domain_uuid),
            username: Set(Some(config.app_name.clone())),
            password: Set(password_hash(config.app_name.clone())?),
            profile: Set(serde_json::to_value(
                StandardClaims::new(SubjectIdentifier::new(user_uuid.to_string()))
                    .set_gender(Some(Gender::Other("unknown".into()))),
            )?),
            created_at: Set(Utc::now()),
            updated_at: Set(Utc::now()),
            ..Default::default()
        })
        .exec(&self.database)
        .await?;

        Ok(user_uuid)
    }
}
