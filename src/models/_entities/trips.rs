//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.7

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "trips")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub trip_id: i64,
    pub direction_id: Option<i64>,
    pub block_id: Option<i64>,
    pub route_id: Option<i64>,
    pub service_id: Option<i64>,
    pub shape_id: Option<i64>,
    pub trip_headsign: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
