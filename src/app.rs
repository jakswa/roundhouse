use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    worker::Processor,
    Result,
};

use crate::controllers;

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::empty().add_route(controllers::trains::routes())
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {}

    async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
        // add a static file route for the /public directory
        let serve_dir = tower_http::services::ServeDir::new("public");
        Ok(router
            .nest_service("/public/:version", serve_dir)
            .layer(tower_http::compression::CompressionLayer::new()))
    }

    fn register_tasks(_tasks: &mut Tasks) {}
}
