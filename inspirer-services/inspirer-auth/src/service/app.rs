use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use url::Url;

use self::app_setting::OIDCSetting;

use super::Service;

pub struct App;

#[derive(Debug, Clone, Serialize, Deserialize, Default, FromJsonQueryResult, PartialEq, Eq)]
pub struct AppSetting {
    pub oidc_setting: OIDCSetting,
}

pub mod app_setting {
    use sea_orm::FromJsonQueryResult;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, FromJsonQueryResult, PartialEq, Eq)]
    pub struct OIDCSetting {
        pub access_token_expire_in: u64,
        pub id_token_expire_in: u64,
        pub refresh_token_expire_in: u64,
        /// 授权码过期时间
        pub authorize_code_expire_in: u64,
    }

    impl Default for OIDCSetting {
        fn default() -> Self {
            OIDCSetting {
                access_token_expire_in: 604800,
                id_token_expire_in: 604800,
                refresh_token_expire_in: 1209600,
                authorize_code_expire_in: 600,
            }
        }
    }
}

impl Service<App> {}
