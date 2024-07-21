use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Stops::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Stops::StopId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Stops::StopCode).big_integer())
                    .col(ColumnDef::new(Stops::StopName).string())
                    .col(ColumnDef::new(Stops::StopLat).double())
                    .col(ColumnDef::new(Stops::StopLon).double())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Trips::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Trips::TripId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Trips::DirectionId).big_integer())
                    .col(ColumnDef::new(Trips::BlockId).big_integer())
                    .col(ColumnDef::new(Trips::RouteId).big_integer())
                    .col(ColumnDef::new(Trips::ServiceId).big_integer())
                    .col(ColumnDef::new(Trips::ShapeId).big_integer())
                    .col(ColumnDef::new(Trips::TripHeadsign).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Routes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Routes::RouteId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Routes::RouteShortName).string())
                    .col(ColumnDef::new(Routes::RouteLongName).string())
                    .col(ColumnDef::new(Routes::RouteDesc).string())
                    .col(ColumnDef::new(Routes::RouteType).big_integer())
                    .col(ColumnDef::new(Routes::RouteUrl).string())
                    .col(ColumnDef::new(Routes::RouteTextColor).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(StopTimes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StopTimes::TripId).big_integer().not_null())
                    .col(
                        ColumnDef::new(StopTimes::StopSequence)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(StopTimes::ArrivalTime).string())
                    .col(ColumnDef::new(StopTimes::DepartureTime).string())
                    .col(ColumnDef::new(StopTimes::StopId).big_integer())
                    .primary_key(
                        sea_query::Index::create()
                            .col(StopTimes::TripId)
                            .col(StopTimes::StopSequence),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(CalendarDates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CalendarDates::Date)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(CalendarDates::ServiceId).big_integer())
                    .col(ColumnDef::new(CalendarDates::ExceptionType).big_integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Calendars::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Calendars::ServiceId)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Calendars::Monday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Tuesday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Wednesday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Thursday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Friday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Saturday).tiny_integer())
                    .col(ColumnDef::new(Calendars::Sunday).tiny_integer())
                    .col(ColumnDef::new(Calendars::StartDate).string())
                    .col(ColumnDef::new(Calendars::EndDate).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stops::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Trips::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Calendars::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(CalendarDates::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Routes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(StopTimes::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Calendars {
    Table,
    ServiceId,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    StartDate,
    EndDate,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CalendarDates {
    Table,
    ServiceId,
    Date,
    ExceptionType,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum StopTimes {
    Table,
    TripId,
    ArrivalTime,
    DepartureTime,
    StopId,
    StopSequence,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Routes {
    Table,
    RouteId,
    RouteShortName,
    RouteLongName,
    RouteDesc,
    RouteType,
    RouteUrl,
    RouteTextColor,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Trips {
    Table,
    TripId,
    DirectionId,
    BlockId,
    ShapeId,
    RouteId,
    ServiceId,
    TripHeadsign,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Stops {
    Table,
    StopId,
    StopCode,
    StopName,
    StopLat,
    StopLon,
}
