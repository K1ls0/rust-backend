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
        pub lat: f64,
        pub long: f64,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AnonymousGeoData  {
    pub status: Status,
    pub loc: GeoLocation,
    pub refresh_time: NaiveDateTime
}

impl AnonymousGeoData {
    pub fn new(
        loc: GeoLocation,
        refresh_time: NaiveDateTime
    ) -> AnonymousGeoData {
        AnonymousGeoData {
            status: Status {
                title: String::from("Ok"),
                code: 200
            },
            loc,
            refresh_time
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoData {
    pub status: Status,
    pub uuid: Uuid,
    pub loc: GeoLocation,
    pub refresh_time: NaiveDateTime,
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


#[derive(Debug, Serialize, Deserialize)]
pub struct GeoDataCollection {
    pub items: std::collections::LinkedList<GeoData>
}
