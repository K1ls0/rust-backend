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
pub struct AnonymGeoLocation {
        pub lat: f64,
        pub long: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoLocation {
        pub id: u32,
        pub lat: f64,
        pub long: f64,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct RXGeoData {
    pub uuid: Uuid,
    pub loc: AnonymGeoLocation,
    pub refresh_time: NaiveDateTime,
}
impl RXGeoData {
    pub fn new(
        uuid: Uuid,
        loc: AnonymGeoLocation,
        refresh_time: NaiveDateTime
    ) -> RXGeoData {
        RXGeoData {
            uuid,
            loc,
            refresh_time
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TXGeoData {
    pub status: Status,
    pub refresh_time: NaiveDateTime,
    pub items: std::collections::LinkedList<GeoLocation>,
}

impl TXGeoData {
    pub fn new(
        refresh_time: NaiveDateTime,
        items: std::collections::LinkedList<GeoLocation>,
    ) -> TXGeoData {
        TXGeoData {
            status: Status {
                title: String::from("Ok"),
                code: 200
            },
            refresh_time,
            items,
        }
    }
}
