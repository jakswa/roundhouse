use async_trait::async_trait;
use axum::{Extension, Router as AxumRouter};
use loco_rs::prelude::*;
use sea_orm::{Database, DatabaseConnection};

use migration::{Migrator, MigratorTrait};

pub struct DatabaseInitializer;

impl DatabaseInitializer {
    pub async fn init() -> DatabaseConnection {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var not set");
        Database::connect(db_url).await.expect("db connection")
    }
}

#[async_trait]
impl Initializer for DatabaseInitializer {
    fn name(&self) -> String {
        "database".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        let db = DatabaseInitializer::init().await;
        Migrator::up(&db, None).await.expect("migrations");
        Ok(router.layer(Extension(db)))
    }
}
