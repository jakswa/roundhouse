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
    pub fn is_arriving(&self) -> bool {
        self.waiting_time == "Arriving"
    }

    pub fn wait_min(&self) -> String {
        let secs = self.waiting_seconds.parse::<i64>().unwrap();
        format!(":{:02}", secs / 60)
    }

    pub fn train_color(&self) -> &str {
        match self.line.as_ref() {
            "RED" => "bg-red-400 border-red-500",
            "GOLD" => "bg-yellow-400 border-yellow-500",
            "GREEN" => "bg-green-400 border-green-500",
            "BLUE" => "bg-blue-400 border-blue-500",
            _ => "violet-700",
        }
    }
    // station fields all end with " STATION" -- kinda redundant huh
    pub fn station_name(&self) -> String {
        let rind = self.station.rfind(' ').unwrap_or(self.station.len());
        self.station[0..rind].to_lowercase()
    }

    pub fn is_destination(&self) -> bool {
        self.station
            .find(&self.destination.to_ascii_uppercase())
            .unwrap_or(1)
            == 0
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
    reqwest::Client::builder()
        // ugh always have SSL probs with itsmarta.com
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap()
        .get("http://developer.itsmarta.com/RealtimeTrain/RestServiceNextTrain/GetRealtimeArrivals")
        .send()
        .await?
        .json()
        .await
}

pub async fn single_station_arrivals(station_name: &String) -> Vec<TrainArrival> {
    let upper_station = station_name.to_ascii_uppercase();
    let mut list: Vec<TrainArrival> = arrivals()
        .await
        .unwrap_or(vec![])
        .into_iter()
        .filter(|arr| arr.station == upper_station)
        .collect();
    list.sort_by_key(|arr| arr.waiting_seconds.parse::<i64>().unwrap());
    list
}

pub async fn single_train_arrivals(train_id: &String) -> Vec<TrainArrival> {
    let mut list: Vec<TrainArrival> = arrivals()
        .await
        .unwrap_or(vec![])
        .into_iter()
        .filter(|arr| &arr.train_id == train_id)
        .collect();
    list.sort_by_key(|arr| arr.waiting_seconds.parse::<i64>().unwrap());
    list
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
            let station_name = vec.last().unwrap().station.to_lowercase();
            // show arrivals for the station in consistent order
            vec.sort_by_key(|arr| match arr.direction.as_ref() {
                "N" => 0,
                "S" => 1,
                "E" => 2,
                _w => 3,
            });
            res.push(Station {
                arrivals: vec.drain(..).collect(),
                name: station_name,
            });
            vec.push(arrival.clone());
        }
    }
    res
}
