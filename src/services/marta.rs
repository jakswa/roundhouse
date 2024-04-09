use cached::proc_macro::once;
use serde::Deserialize;

// stations with their approximate location.
// could also get lat/long from the GTFS CSVs one day.
pub const STATIONS: [(&str, f64, f64); 38] = [
    ("airport", 33.639975, -84.44403199999999),
    ("arts center", 33.789283, -84.387125),
    ("ashby", 33.756289, -84.41755599999999),
    ("avondale", 33.77533, -84.280715),
    ("bankhead", 33.772979, -84.428537),
    ("brookhaven", 33.859928, -84.33922),
    ("buckhead", 33.847874, -84.367296),
    ("chamblee", 33.8879695, -84.30468049999999),
    ("civic center", 33.766245, -84.38750399999999),
    ("college park", 33.6513813, -84.4470162),
    ("decatur", 33.774455, -84.297131),
    ("doraville", 33.9026881, -84.28025099999999),
    ("dunwoody", 33.9486029, -84.355848),
    ("east lake", 33.765062, -84.31261099999999),
    ("east point", 33.676609, -84.440595),
    ("edgewood candler park", 33.761812, -84.340064),
    ("five points", 33.754061, -84.391539),
    ("garnett", 33.748938, -84.395513),
    ("georgia state", 33.749732, -84.38569700000001),
    ("hamilton e holmes", 33.7545107, -84.4722046),
    ("indian creek", 33.769212, -84.229255),
    ("inman park", 33.757317, -84.35262),
    ("kensington", 33.772093, -84.252217),
    ("king memorial", 33.749468, -84.37601099999999),
    ("lakewood", 33.700649, -84.429541),
    ("lenox", 33.845137, -84.357854),
    ("lindbergh", 33.823698, -84.369248),
    ("medical center", 33.9106263, -84.3513751),
    ("midtown", 33.780737, -84.386657),
    ("north ave", 33.771696, -84.387411),
    ("north springs", 33.9452199, -84.3572593),
    ("oakland city", 33.71726400000001, -84.42527899999999),
    ("omni dome", 33.7489954, -84.3879824),
    ("peachtree center", 33.759532, -84.387564),
    ("sandy springs", 33.9321044, -84.3513524),
    ("vine city", 33.756612, -84.404348),
    ("west end", 33.73584, -84.412967),
    ("west lake", 33.7533436, -84.4475581),
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

    let mut stations = STATIONS.into_iter().map(|i| i.0);
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
