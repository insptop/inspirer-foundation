use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// This is a default app name use for auth service
    pub app_name: String,
    /// The url is endpoint of the first app (service)
    pub app_endpoint: Url,
}
