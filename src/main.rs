#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate rusqlite;

mod handlers;
mod types;
mod sqlite_adapter;

use std::sync::Mutex;

use actix_web::{
    web, 
    App, 
    HttpServer,
    middleware::Logger,
};

use types::AnonymGeoLocation;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    //let cfg_file = File::open("./service_config.yml")?;

    HttpServer::new(move || {
        debug!("Initializing Http Server");
        // let data = sqlite_adapter::SQLDataBase::new(String::from("./db/geoloc-data.db"), String::from("geoloc_storage"));
        let data = sqlite_adapter::SQLDataBase::new(&String::from("./db/geoloc-data.db"), &String::from("geoloc_storage"));

        App::new()
            .wrap(Logger::default()) // Enable logging
            .data(AnonymGeoLocation {lat: 0.4, long: 1.3})
            .service(
                web::resource("/geoloc/publishLocation")
                .route(web::post().to(handlers::geoloc::add_location))
            )
            .service(
                web::resource("/geoloc/locations")
                .route(web::get().to(handlers::geoloc::get_locations))
            )
            .service(
                web::resource("/geoloc")
                .route(web::get().to(handlers::geoloc::get_locations))
                .route(web::post().to(handlers::geoloc::add_location))
            ).default_service(
                web::route().to(handlers::geoloc::render_404)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
