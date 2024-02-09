#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::views::trains::*;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};

use axum_extra::extract::cookie::{Cookie, CookieJar};
use http::header;

async fn trains_index(cookies: CookieJar) -> impl IntoResponse {
    let stations = crate::services::marta::arrivals_by_station().await;
    let starred_station_names = cookies
        .get("starred_stations")
        .map(|cookie| {
            cookie
                .value()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![]);
    let starred_stations = stations
        .iter()
        .filter(|s| {
            starred_station_names
                .iter()
                .any(|ss| ss.starts_with(&s.name.to_ascii_uppercase()))
        })
        .map(|s| s.clone())
        .collect::<Vec<crate::services::marta::Station>>();

    (
        [(header::CACHE_CONTROL, "no-store")],
        super::HtmlTemplate(TrainsIndexResponse {
            stations,
            starred_stations,
        }),
    )
}

async fn trains_station(
    cookies: CookieJar,
    Path(station_name): Path<String>,
) -> axum::response::Response {
    let starred_station_names = cookies
        .get("starred_stations")
        .map(|cookie| {
            cookie
                .value()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![]);
    if !station_name
        .rfind(' ')
        .is_some_and(|ind| crate::services::marta::STATIONS.contains(&&station_name[0..ind]))
    {
        return http404();
    }
    let upcase_station = station_name.to_ascii_uppercase();
    let arrivals = crate::services::marta::single_station_arrivals(&station_name).await;
    (
        [(header::CACHE_CONTROL, "no-store")],
        super::HtmlTemplate(TrainsStationResponse {
            arrivals,
            station_name,
            is_starred: starred_station_names.contains(&upcase_station),
        }),
    )
        .into_response()
}

async fn star_station(cookies: CookieJar, Path(station_name): Path<String>) -> impl IntoResponse {
    let mut starred_station_names = cookies
        .get("starred_stations")
        .map(|cookie| {
            cookie
                .value()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![]);
    starred_station_names.push(station_name.to_ascii_uppercase());
    let mut cookie = Cookie::new("starred_stations", starred_station_names.join(","));
    cookie.set_path("/");
    cookie.set_max_age(::cookie::time::Duration::days(365));
    (cookies.add(cookie), Redirect::to("/"))
}

async fn unstar_station(cookies: CookieJar, Path(station_name): Path<String>) -> impl IntoResponse {
    let mut starred_station_names = cookies
        .get("starred_stations")
        .map(|cookie| {
            cookie
                .value()
                .split(",")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or(vec![]);
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
