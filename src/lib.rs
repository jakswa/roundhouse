pub mod app;
pub mod controllers;
pub mod initializers;
pub mod mailers;
pub mod models;
pub mod services;
pub mod tasks;
pub mod views;
pub mod workers;

pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}
