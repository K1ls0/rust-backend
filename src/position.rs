pub mod types {
    use uuid::Uuid;
    use std::time;
    use status::status;

    
    pub struct BareStatus {
        status: status
    }

    impl BareStatus {
        fn new(title: String, code: i32) {
            BareStatus {
                status: status {
                    title,
                    code
                },

            }
        }
    }

    pub struct GeoLocation {
        status: status,
        uuid: Uuid,
        lattitude: f64,
        longitude: f64,
        refresh_time: time
    }

    #[derive(BareStatus)]
    impl GeoLocation {
        fn new(
            uuid: Uuid,
            lat: f64,
            long: f64,
            refresh_time: time
        ) {
            GeoLocation {
                status: status {
                    title: String::from("Ok"),
                    code: 200
                },
                uuid: uuid,
                lattitude: lat,
                longitude: long,
                refresh_time: time
            }
        }

    }

}
