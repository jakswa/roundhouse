use crate::models::_entities::{
    calendar_dates, calendars, routes, shapes, stop_times, stops, trips,
};
use loco_rs::prelude::*;

use std::fs::File;
use std::io::prelude::*;

const MARTA_GTFS_URL_DEFAULT: &str = "https://itsmarta.com/google_transit_feed/google_transit.zip";

pub struct IngestSchedule;
#[async_trait]
impl Task for IngestSchedule {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "ingest_schedule".to_string(),
            detail: "Ingest MARTA's GTFS CSV containing schedule data".to_string(),
        }
    }
    async fn run(&self, app_context: &AppContext, _vars: &task::Vars) -> Result<()> {
        for (filename, file) in csv_extraction().await.iter() {
            tracing::info!("processing {}", filename);
            let mut reader = csv::Reader::from_reader(file);
            let tx = app_context.db.begin().await?;
            match filename.as_str() {
                "calendar.txt" => {
                    for result in reader.deserialize() {
                        let model: calendars::Model = result.expect("CSV parsing works right?");
                        calendars::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "calendar_dates.txt" => {
                    for result in reader.deserialize() {
                        let model: calendar_dates::Model =
                            result.expect("CSV parsing works right?");
                        calendar_dates::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "routes.txt" => {
                    for result in reader.deserialize() {
                        let model: routes::Model = result.expect("CSV parsing works right?");
                        routes::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "stop_times.txt" => {
                    for result in reader.deserialize() {
                        let model: stop_times::Model = result.expect("CSV parsing works right?");
                        stop_times::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "shapes.txt" => {
                    for result in reader.deserialize() {
                        let model: shapes::Model = result.expect("CSV parsing works right?");
                        shapes::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "stops.txt" => {
                    for result in reader.deserialize() {
                        let model: stops::Model = result.expect("CSV parsing works right?");
                        stops::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                "trips.txt" => {
                    for result in reader.deserialize() {
                        let model: trips::Model = result.expect("CSV parsing works right?");
                        trips::Entity::insert(model.into_active_model())
                            .on_conflict(sea_query::OnConflict::new().do_nothing().to_owned())
                            .exec_without_returning(&tx)
                            .await?;
                    }
                }
                _ => {}
            }
            tx.commit().await?;
        }
        tracing::info!(":: finished ingesting! ::");
        Ok(())
    }
}

// this goes out of its way to copy to a tempfile
// because the ZipFile objects don't implement Send
async fn csv_extraction() -> Vec<(String, File)> {
    let file = gtfs_zip_file().await;
    let names = vec![
        "calendar.txt",
        "calendar_dates.txt",
        "routes.txt",
        "stop_times.txt",
        "shapes.txt",
        "stops.txt",
        "trips.txt",
    ];
    let mut archive = zip::read::ZipArchive::new(file).expect("zip archive init");
    names
        .iter()
        .map(|filename| {
            let mut csv_file = archive.by_name(filename).expect("calendar in zip");
            let mut other_file = tempfile::tempfile().expect("tempfiles work");
            std::io::copy(&mut csv_file, &mut other_file).expect("copying files");
            other_file.rewind().expect("rewinding");
            (filename.to_string(), other_file)
        })
        .collect()
}

async fn gtfs_zip_file() -> File {
    let url = std::env::var("MARTA_GTFS_URL").unwrap_or(MARTA_GTFS_URL_DEFAULT.to_string());
    let mut file = tempfile::tempfile().expect("tempfiles");
    tracing::info!("downloading ZIP from {}", url);
    let res = reqwest::get(url).await.expect("http get");
    let bytes = res.bytes().await.expect("bytes from http get");
    std::io::copy(&mut bytes.as_ref(), &mut file).expect("io copy works");
    file.rewind().expect("be kind");
    tracing::info!("-> done downloading zip!");
    file
}
