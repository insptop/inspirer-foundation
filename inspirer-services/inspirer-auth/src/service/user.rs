use inspirer_framework::{http::StatusCode, preludes::*, response::ErrorDetail};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::{auth::user::UserCredential, entity::users, password::password_verify};

use super::Service;

pub struct User;

impl Service<User> {
    pub async fn find_user_by_credential(
        &self,
        credential: UserCredential,
    ) -> Result<users::Model> {
        let mut query = users::Entity::find();

        let password = match credential {
            UserCredential::Username { username, password } => {
                tracing::debug!(
                    username = username,
                    password = password,
                    "login use username credential"
                );
                query = query.filter(users::Column::Username.eq(username));
                password
            }
            UserCredential::Email { email, password } => {
                query = query.filter(users::Column::Email.eq(email));
                password
            }
        };

        let user = query.one(&self.database).await?.ok_or(Error::CustomError(
            StatusCode::NOT_FOUND,
            ErrorDetail::with_reason("User not exists"),
        ))?;

        if password_verify(password, &user.password).is_err() {
            return Err(Error::Unauthorized(
                "User not exists or password error".into(),
            ));
        }

        Ok(user)
    }
}
