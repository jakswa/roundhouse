use loco_rs::prelude::*;

use crate::views::home::HomeResponse;
use axum::response::IntoResponse;

async fn home_index() -> impl IntoResponse {
    super::HtmlTemplate(HomeResponse {
        app_name: "loco".to_string(),
    })
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(home_index))
}
