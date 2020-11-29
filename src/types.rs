use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    title: String,
    code: i32
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
        lat: f64,
        long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoData {
    status: Status,
    uuid: Uuid,
    loc: GeoLocation,
    refresh_time: NaiveDateTime,
}

impl GeoData {
    pub fn new(
        uuid: Uuid,
        loc: GeoLocation,
        refresh_time: NaiveDateTime
    ) -> GeoData {
        GeoData {
            status: Status {
                title: String::from("Ok"),
                code: 200
            },
            uuid,
            loc,
            refresh_time
        }
    }

}

