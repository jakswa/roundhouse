// proof of concept for when i need GTFS data.
// is able to import the 100MB stop_times CSV in ~20MB
// (which is far and away the biggest file in the GTFS ZIP archive)

use std::io::Write;

use migration::{Migrator, MigratorTrait};
use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, TransactionTrait};
use zip::read::ZipArchive; // needed for is_empty()

use entity::calendar_dates::{self, Entity as CalendarDates};
use entity::calendars::{self, Entity as Calendars};
use entity::routes::{self, Entity as Routes};
use entity::shapes::{self, Entity as Shapes};
use entity::stop_times::{self, Entity as StopTimes};
use entity::stops::{self, Entity as Stops};
use entity::trips::{self, Entity as Trips};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let mut opts =
        sea_orm::ConnectOptions::new(std::env::var("DATABASE_URL").expect("set DATABASE_URL"));
    opts.sqlx_logging(false);
    let connection = sea_orm::Database::connect(opts).await?;
    Migrator::up(&connection, None).await?;

    tracing::info!("checking GTFS file, downloading if needed.");
    let file = gtfs_zip_file(&std::env::var("GTFS_ZIP_URL").expect("set GTFS_ZIP_URL env")).await?;
    tracing::info!("file acquired. trying to unzip...");
    let mut archive = ZipArchive::new(file)?;

    tracing::info!("importing stop times.");
    {
        let csv_file = archive.by_name("stop_times.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: stop_times::Model = result?;
            let active_model = model.into_active_model();
            StopTimes::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }
        tx.commit().await?;
    }

    tracing::info!("importing trips.");
    {
        let csv_file = archive.by_name("trips.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: trips::Model = result?;
            let active_model = model.into_active_model();
            Trips::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }

    tracing::info!("importing calendar.");
    {
        let csv_file = archive.by_name("calendar.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: calendars::Model = result?;
            let active_model = model.into_active_model();
            Calendars::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }

    tracing::info!("importing calendar_dates.");
    {
        let csv_file = archive.by_name("calendar_dates.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: calendar_dates::Model = result?;
            let active_model = model.into_active_model();
            CalendarDates::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }

    tracing::info!("importing routes.");
    {
        let csv_file = archive.by_name("routes.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: routes::Model = result?;
            let active_model = model.into_active_model();
            Routes::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }

    tracing::info!("importing stops.");
    {
        let csv_file = archive.by_name("stops.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: stops::Model = result?;
            let active_model = model.into_active_model();
            Stops::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }

    tracing::info!("importing shapes.");
    {
        let csv_file = archive.by_name("shapes.txt")?;
        let mut reader = csv::Reader::from_reader(csv_file);
        let tx = connection.begin().await?;
        for result in reader.deserialize() {
            let model: shapes::Model = result?;
            let active_model = model.into_active_model();
            Shapes::insert(active_model)
                .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                .exec_without_returning(&tx)
                .await?;
        }

        tx.commit().await?;
    }
    tracing::info!("done importing.");
    Ok(())
}

async fn gtfs_zip_file(url: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let zip_dir = dirs().data_dir().join("gtfs.zip");
    tracing::info!("zip file at {}", zip_dir.to_str().unwrap());
    if let Ok(file) = std::fs::File::open(&zip_dir) {
        return Ok(file);
    }

    {
        let res = reqwest::get(url).await?;
        std::fs::create_dir_all(dirs().data_dir())?;
        let mut file = std::fs::File::create(&zip_dir)?;
        let bytes = res.bytes().await?;
        std::io::copy(&mut bytes.as_ref(), &mut file)?;
        file.flush()?;
    }

    Ok(std::fs::File::open(&zip_dir).unwrap())
}

fn dirs() -> directories::ProjectDirs {
    directories::ProjectDirs::from("io", "marta", "roundhouse").unwrap()
}
