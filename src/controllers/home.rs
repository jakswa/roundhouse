use loco_rs::prelude::*;

use crate::views::home::HomeResponse;
use axum::response::IntoResponse;

async fn home_index() -> impl IntoResponse {
    super::HtmlTemplate(HomeResponse {
        stations: crate::services::marta::arrivals_by_station().await,
    })
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(home_index))
}
