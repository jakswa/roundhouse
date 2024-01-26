use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

pub struct AssetsInitializer;

#[async_trait]
impl Initializer for AssetsInitializer {
    fn name(&self) -> String {
        "assets".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let serve_dir = tower_http::services::ServeDir::new("public");
        Ok(router
            .nest_service("/public/:version", serve_dir)
            .layer(tower_http::compression::CompressionLayer::new()))
    }
}
