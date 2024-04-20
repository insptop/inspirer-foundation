use std::env::current_dir;

use inspirer_framework::{
    preludes::*,
    tower_http::services::{ServeDir, ServeFile},
};

use crate::app::App;

pub fn routes() -> Router<App> {
    let path = current_dir()
        .unwrap()
        .join("inspirer-services/inspirer-auth/auth-page/dist");

    assert!(path.exists());
    assert!(path.join("index.html").exists());

    Router::new()
        .route_service("/vite.svg", ServeFile::new(path.join("vite.svg")))
        .route_service("/login", ServeFile::new(path.join("index.html")))
        .nest_service("/assets", ServeDir::new(path.join("assets")))
}
