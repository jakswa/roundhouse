use cached::proc_macro::once;
use serde::Deserialize;

const STATIONS: [&str; 38] = [
    "airport",
    "arts center",
    "ashby",
    "avondale",
    "bankhead",
    "brookhaven",
    "buckhead",
    "chamblee",
    "civic center",
    "college park",
    "decatur",
    "doraville",
    "dunwoody",
    "east lake",
    "east point",
    "edgewood candler park",
    "five points",
    "garnett",
    "georgia state",
    "hamilton e holmes",
    "indian creek",
    "inman park",
    "kensington",
    "king memorial",
    "lakewood",
    "lenox",
    "lindbergh",
    "medical center",
    "midtown",
    "north ave",
    "north springs",
    "oakland city",
    "omni dome",
    "peachtree center",
    "sandy springs",
    "vine city",
    "west end",
    "west lake",
];

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

#[derive(Clone)]
pub struct Station {
    pub name: String,
    pub arrivals: Vec<TrainArrival>,
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

    let mut stations = STATIONS.into_iter();
    let mut curr_station = stations.next().unwrap();

    for arrival in arrivals.drain(..) {
        let arrival_station = arrival.station_name();
        if curr_station == arrival_station {
            if !vec.iter().any(|arr| arrival.direction == arr.direction) {
                vec.push(arrival.clone());
            }
        } else {
            // show arrivals for the station in consistent order
            vec.sort_by_key(|arr| match arr.direction.as_ref() {
                "N" => 0,
                "S" => 1,
                "E" => 2,
                _w => 3,
            });
            loop {
                // cycle/add stations until we find next
                res.push(Station {
                    arrivals: vec,
                    name: curr_station.to_string(),
                });
                curr_station = stations.next().unwrap();
                if curr_station == arrival_station {
                    break;
                }
                vec = vec![];
            }
            vec = vec![arrival.clone()];
        }
    }
    loop {
        // cycle/add stations until no more stations.
        res.push(Station {
            arrivals: vec,
            name: curr_station.to_string(),
        });
        match stations.next() {
            None => break,
            Some(next_station) => curr_station = next_station,
        }
        vec = vec![];
    }
    res
}
