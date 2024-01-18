use cached::proc_macro::once;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TrainArrival {
    pub destination: String,
    pub direction: String,
    pub event_time: String,
    pub line: String,
    pub next_arr: String,
    pub station: String,
    pub train_id: String,
    pub waiting_seconds: String,
    pub waiting_time: String,
}

impl TrainArrival {
    pub fn wait_min(&self) -> String {
        let secs = self.waiting_seconds.parse::<i64>().unwrap();
        format!(":{:02}", secs / 60)
    }
}

pub struct Station {
    pub name: String,
    pub arrivals: Vec<TrainArrival>,
}

impl Station {
    // station fields all end with " STATION" -- kinda redundant huh
    pub fn station_name(&self) -> String {
        let rind = self.name.rfind(' ').unwrap_or(self.name.len());
        self.name[0..rind].to_lowercase()
    }
}

#[once(time = 10, result = true, sync_writes = true)]
pub async fn arrivals() -> Result<Vec<TrainArrival>, reqwest::Error> {
    reqwest::get(
        "http://developer.itsmarta.com/RealtimeTrain/RestServiceNextTrain/GetRealtimeArrivals",
    )
    .await?
    .json()
    .await
}

pub async fn arrivals_by_station() -> Vec<Station> {
    let mut res: Vec<Station> = vec![];
    let mut vec: Vec<TrainArrival> = vec![];
    let mut arrivals = arrivals().await.unwrap();
    arrivals.sort_by(|a, b| {
        if a.station == b.station {
            a.waiting_seconds
                .parse::<i64>()
                .unwrap()
                .cmp(&b.waiting_seconds.parse::<i64>().unwrap())
        } else {
            a.station.cmp(&b.station)
        }
    });

    for arrival in arrivals.drain(..) {
        if vec.is_empty() || vec.last().unwrap().station == arrival.station {
            if !vec.iter().any(|arr| arrival.direction == arr.direction) {
                vec.push(arrival.clone());
            }
        } else {
            let station_name = vec.last().unwrap().station.clone();
            res.push(Station {
                arrivals: vec.drain(..).collect(),
                name: station_name,
            });
            vec.push(arrival.clone());
        }
    }
    res
}
