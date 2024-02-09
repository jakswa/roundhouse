use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

use crate::controllers::HtmlTemplate;
use crate::views::Http404Template;
use axum::response::IntoResponse;

pub struct GlobalNotFound;

#[async_trait]
impl Initializer for GlobalNotFound {
    fn name(&self) -> String {
        "global_not_found".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router.fallback(generic_404))
    }
}

async fn generic_404() -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        HtmlTemplate(Http404Template::default()),
    )
}
