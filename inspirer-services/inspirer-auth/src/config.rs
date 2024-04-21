use axum_login::tower_sessions::Expiry;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    /// This is a default app name use for auth service
    pub app_name: String,

    /// The url is endpoint of the first app (service)
    pub app_endpoint: Url,

    /// Auth session config
    pub session: SessionConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionConfig {
    /// Session driver setting
    #[serde(flatten)]
    pub driver: SessionDriverConfig,

    /// The name of the cookie used for the session
    pub session_name: Option<String>,

    /// Configures the `"Max-Age"` attribute of the cookie used for the session.
    pub session_expiry: Option<Expiry>,

    /// Configures the `"Secure"` attribute of the cookie used for the session.
    pub with_secure: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "driver", rename_all = "snake_case")]
pub enum SessionDriverConfig {
    /// Use memory as session store
    Memory,

    /// Use redis as session store
    Redis {
        /// Redis connection url
        ///
        /// # URL Syntax
        ///
        /// **Centralized**
        ///
        /// ```text
        /// redis|rediss :// [[username:]password@] host [:port][/database]
        /// ```
        ///
        /// **Clustered**
        ///
        /// ```text
        /// redis|rediss[-cluster] :// [[username:]password@] host [:port][?[node=host1:port1][&node=host2:port2][&node=hostN:portN]]
        /// ```
        ///
        /// **Sentinel**
        ///
        /// ```text
        /// redis|rediss[-sentinel] :// [[username1:]password1@] host [:port][/database][?[node=host1:port1][&node=host2:port2][&node=hostN:portN]
        ///                             [&sentinelServiceName=myservice][&sentinelUsername=username2][&sentinelPassword=password2]]
        /// ```
        ///
        /// # Schemes
        ///
        /// This function will use the URL scheme to determine which server type the caller is using. Valid schemes include:
        ///
        /// * `redis` - TCP connected to a centralized server.
        /// * `rediss` - TLS connected to a centralized server.
        /// * `redis-cluster` - TCP connected to a cluster.
        /// * `rediss-cluster` - TLS connected to a cluster.
        /// * `redis-sentinel` - TCP connected to a centralized server behind a sentinel layer.
        /// * `rediss-sentinel` - TLS connected to a centralized server behind a sentinel layer.
        ///
        /// **The `rediss` scheme prefix requires the `enable-native-tls` or `enable-rustls` feature.**
        ///
        /// # Query Parameters
        ///
        /// In some cases it's necessary to specify multiple node hostname/port tuples (with a cluster or sentinel layer for
        /// example). The following query parameters may also be used in their respective contexts:
        ///
        /// * `node` - Specify another node in the topology. In a cluster this would refer to any other known cluster node.
        ///   In the context of a Redis sentinel layer this refers to a known **sentinel** node. Multiple `node` parameters
        ///   may be used in a URL.
        /// * `sentinelServiceName` - Specify the name of the sentinel service. This is required when using the
        ///   `redis-sentinel` scheme.
        /// * `sentinelUsername` - Specify the username to use when connecting to a **sentinel** node. This requires the
        ///   `sentinel-auth` feature and allows the caller to use different credentials for sentinel nodes vs the actual
        ///   Redis server. The `username` part of the URL immediately following the scheme will refer to the username used
        ///   when connecting to the backing Redis server.
        /// * `sentinelPassword` - Specify the password to use when connecting to a **sentinel** node. This requires the
        ///   `sentinel-auth` feature and allows the caller to use different credentials for sentinel nodes vs the actual
        ///   Redis server. The `password` part of the URL immediately following the scheme will refer to the password used
        ///   when connecting to the backing Redis server.
        ///
        /// See the [from_url_centralized](Self::from_url_centralized), [from_url_clustered](Self::from_url_clustered), and
        /// [from_url_sentinel](Self::from_url_sentinel) for more information. Or see the [RedisConfig](Self) unit tests for
        /// examples.
        database_url: String,

        /// Redis pool size
        pool_size: usize,
    },
}
