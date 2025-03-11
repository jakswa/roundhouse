use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{
    controllers,
    tasks,
    // models::_entities::users, workers::downloader::DownloadWorker
};

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

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(crate::initializers::assets::AssetsInitializer),
            //Box::new(initializers::view_engine::ViewEngineInitializer,)
        ])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::gtfs::routes())
            .add_route(controllers::trains::routes())
            .add_route(controllers::bus_routes::routes())
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        // queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
        tasks.register(tasks::ingest_schedule::IngestSchedule);
    }
    async fn truncate(_ctx: &AppContext) -> Result<()> {
        //truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(_ctx: &AppContext, _base: &Path) -> Result<()> {
        //db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
        //    .await?;
        Ok(())
    }
}
