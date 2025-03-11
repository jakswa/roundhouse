use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

use crate::controllers::HtmlTemplate;
use crate::views::Http404Template;
use axum::response::IntoResponse;

pub struct AssetsInitializer;

#[async_trait]
impl Initializer for AssetsInitializer {
    fn name(&self) -> String {
        "assets".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let serve_dir = tower_http::services::ServeDir::new("public/assets").precompressed_gzip();
        let serve_images = tower_http::services::ServeDir::new("public/images");
        Ok(router
            .layer(tower_http::compression::CompressionLayer::new().no_br())
            // static things go after compression layer.
            // - pre-compressing css/js/etc on deploy
            // - images don't need compression usually
            .nest_service(
                "/public/{version}",
                serve_dir.fallback(axum::Router::new().fallback(generic_404)),
            )
            .nest_service(
                "/images",
                serve_images.fallback(axum::Router::new().fallback(generic_404)),
            )
            .fallback(generic_404))
    }
}

async fn generic_404() -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        HtmlTemplate(Http404Template::default()),
    )
}
