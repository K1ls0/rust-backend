use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize)]
pub struct Status {
    title: String,
    code: i32
}

#[derive(Serialize, Deserialize)]
pub struct BareStatus {
    status: Status
}

impl BareStatus {
    pub fn new(title: String, code: i32) -> BareStatus {
        BareStatus {
            status: Status {
                title,
                code
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GeoLocation {
    status: Status,
    uuid: Uuid,
    lattitude: f64,
    longitude: f64,
    refresh_time: NaiveDateTime,
}

impl GeoLocation {
    pub fn new(
        uuid: Uuid,
        lat: f64,
        long: f64,
        refresh_time: NaiveDateTime
    ) -> GeoLocation {
        GeoLocation {
            status: Status {
                title: String::from("Ok"),
                code: 200
            },
            uuid,
            lattitude: lat,
            longitude: long,
            refresh_time
        }
    }

}

