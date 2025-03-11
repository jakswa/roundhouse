#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20230527_000148_init_db;
mod m20230721_002803_create_shapes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230527_000148_init_db::Migration),
            Box::new(m20230721_002803_create_shapes::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
