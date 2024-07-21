use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Shapes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Shapes::ShapeId).integer().not_null())
                    .col(ColumnDef::new(Shapes::ShapePtLat).double().not_null())
                    .col(ColumnDef::new(Shapes::ShapePtLon).double().not_null())
                    .col(ColumnDef::new(Shapes::ShapePtSequence).integer().not_null())
                    .col(
                        ColumnDef::new(Shapes::ShapeDistTraveled)
                            .double()
                            .not_null(),
                    )
                    .primary_key(
                        sea_query::Index::create()
                            .col(Shapes::ShapeId)
                            .col(Shapes::ShapePtSequence),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Shapes::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Shapes {
    Table,
    ShapeId,
    ShapePtLat,
    ShapePtLon,
    ShapePtSequence,
    ShapeDistTraveled,
}
