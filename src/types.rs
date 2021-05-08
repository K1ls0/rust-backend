
pub mod rest {
    use serde::{ Serialize, Deserialize };
    use uuid::Uuid;
    use chrono::NaiveDateTime;

    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct Status {
        pub title: String,
        pub code: i32
    }

    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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

    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct GeoLocation {
            pub lat: f32,
            pub long: f32,
    }

    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct GeoData {
        pub uuid: Uuid,
        pub loc: GeoLocation,
        pub refresh_time: NaiveDateTime,
    }

    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct GeoReturnData {
        pub status: Status,
        pub data: GeoData,
    }

    impl GeoReturnData {
        pub fn new(
            uuid: Uuid,
            loc: GeoLocation,
            refresh_time: NaiveDateTime
        ) -> GeoReturnData {
            GeoReturnData {
                status: Status {
                    title: String::from("Ok"),
                    code: 200
                },
                data: GeoData {
                    uuid,
                    loc,
                    refresh_time
                }
            }
        }
    }


    #[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
    pub struct GeoDataList {
        pub items: Vec<GeoData>
    }


    #[derive(Clone, PartialEq, Debug, Deserialize)]
    pub struct PathUuid {
        pub uuid: uuid::Uuid,
    }
}

