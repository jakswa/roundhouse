//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "stop_times")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub trip_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub stop_sequence: i64,
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub stop_id: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
