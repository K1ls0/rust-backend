use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct GeoLocData {
    pub client_uuid: String,
    pub lattitude: f32,
    pub longitude: f32,
    pub refresh: NaiveDateTime,
    pub notes: Option<String>,
}
