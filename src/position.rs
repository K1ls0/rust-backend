pub mod geo {
    use uuid::Uuid;
    use std::time;

    pub struct GeoLocation {
        uuid: Uuid,
        lattitude: f64,
        longitude: f64,
        refresh_time: time
    }

    impl GeoLocation {
        fn new(
            uuid: Uuid,
            lat: f64,
            long: f64,
            refresh_time: time
        ) {
            GeoLocation {
                uuid: uuid,
                lattitude: lat,
                longitude: long,
                refresh_time: time
            }
        }

    }
}
