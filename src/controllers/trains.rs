#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::services::marta::{Station, STATIONS};
use crate::views::trains::*;
use axum::extract::{Path, Query};
use axum::response::{IntoResponse, Redirect};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use http::header;
use serde::Deserialize;

async fn trains_index(cookies: CookieJar) -> impl IntoResponse {
    let stations = crate::services::marta::arrivals_by_station().await;

    let starred_station_names = starred_station_names(&cookies);
    let starred_stations = stations
        .iter()
        .filter(|s| {
            starred_station_names
                .iter()
                .any(|ss| ss.starts_with(&s.name.to_ascii_uppercase()))
        })
        .map(|s| s.clone())
        .collect::<Vec<Station>>();

    (
        [(header::CACHE_CONTROL, "no-store")],
        super::HtmlTemplate(TrainsIndexResponse {
            nearby_stations: nearby_stations(&cookies, &stations),
            nearby_enabled: cookies.get("nearby_on").is_some(),
            starred_stations,
            stations,
        }),
    )
}

#[derive(Deserialize)]
struct StationOptions {
    from: Option<String>,
}
async fn trains_station(
    cookies: CookieJar,
    Path(station_name): Path<String>,
    query: Query<StationOptions>,
) -> axum::response::Response {
    let starred_station_names = starred_station_names(&cookies);
    if !station_name.ends_with(" station")
        || !station_name
            .rfind(" station")
            .is_some_and(|ind| STATIONS.into_iter().any(|i| i.0 == &station_name[0..ind]))
    {
        return http404();
    }
    let station = &station_name[0..station_name.rfind(" station").unwrap()];
    let upcase_station = station_name.to_ascii_uppercase();
    let arrivals = crate::services::marta::single_station_arrivals(station).await;
    let train_id = query.0.from.unwrap_or_else(|| String::new());
    let station_with_arrivals = Station {
        arrivals,
        name: station.to_string(),
    };
    (
        [(header::CACHE_CONTROL, "no-store")],
        super::HtmlTemplate(TrainsStationResponse {
            station_with_arrivals,
            is_starred: starred_station_names.contains(&upcase_station),
            train_id,
        }),
    )
        .into_response()
}

async fn star_station(cookies: CookieJar, Path(station_name): Path<String>) -> impl IntoResponse {
    let mut starred_station_names = starred_station_names(&cookies);
    starred_station_names.push(station_name.to_ascii_uppercase());
    let mut cookie = Cookie::new("starred_stations", starred_station_names.join(","));
    cookie.set_path("/");
    cookie.set_max_age(::cookie::time::Duration::days(365));
    (cookies.add(cookie), Redirect::to("/"))
}

async fn unstar_station(cookies: CookieJar, Path(station_name): Path<String>) -> impl IntoResponse {
    let mut starred_station_names = starred_station_names(&cookies);
    starred_station_names = starred_station_names
        .into_iter()
        .filter(|s| s != &station_name.to_ascii_uppercase())
        .collect();
    let mut cookie = Cookie::new("starred_stations", starred_station_names.join(","));
    cookie.set_path("/");
    (cookies.add(cookie), Redirect::to("/"))
}

async fn trains_show(Path(train_id): Path<String>) -> impl IntoResponse {
    if train_id.parse::<u64>().is_err() {
        return http404();
    }
    (
        [(header::CACHE_CONTROL, "no-store")],
        super::HtmlTemplate(TrainsShowResponse {
            arrivals: crate::services::marta::single_train_arrivals(&train_id.clone()).await,
            train_id,
        }),
    )
        .into_response()
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(trains_index))
        .add("/stations/:station_name", get(trains_station))
        .add("/trains/:train_id", get(trains_show))
        .add("/star/:station_name", get(star_station))
        .add("/unstar/:station_name", get(unstar_station))
}

fn http404() -> axum::response::Response {
    (
        axum::http::StatusCode::NOT_FOUND,
        super::HtmlTemplate(crate::views::Http404Template::default()),
    )
        .into_response()
}

fn starred_station_names(cookies: &CookieJar) -> Vec<String> {
    cookies
        .get("starred_stations")
        .map(|cookie| {
            cookie
                .value()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![])
}

fn nearby_stations(cookies: &CookieJar, stations: &Vec<Station>) -> Vec<Station> {
    cookies
        .get("nearby_to")
        .map(|cookie| cookie.value().split_once(","))
        .flatten()
        .map(|(lat, lon)| (lat.parse::<f64>().unwrap(), lon.parse::<f64>().unwrap()))
        .map(|pos| closest_stations(pos, stations))
        .unwrap_or(vec![])
}

fn closest_stations(pos: (f64, f64), stations: &Vec<Station>) -> Vec<Station> {
    let mut station_coords: Vec<&(&str, f64, f64)> = STATIONS.iter().collect();
    station_coords.sort_by(|s1, s2| {
        let v1 = (s1.1 - pos.0).powi(2) + (s1.2 - pos.1).powi(2);
        let v2 = (s2.1 - pos.0).powi(2) + (s2.2 - pos.1).powi(2);
        v1.total_cmp(&v2)
    });
    station_coords
        .iter()
        .take(3)
        .map(|s| {
            stations
                .iter()
                .find(|stat| stat.name == s.0)
                .unwrap()
                .clone()
        })
        .collect()
}
