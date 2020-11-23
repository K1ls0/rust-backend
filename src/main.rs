#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate chrono;

mod handlers;
mod types;

use actix_web::{
    web, 
    App, 
    HttpServer,
    middleware::Logger,
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    //let cfg_file = File::open("./service_config.yml")?;


    HttpServer::new(move || {
        debug!("Initializing Http Server");
        App::new()
            .wrap(Logger::default()) // Enable logging
            .service(
                web::resource("/geoloc/publishLocation")
                .route(web::post().to(handlers::geoloc::add_location))
            )
            .service(
                web::resource("/geoloc/locations")
                .route(web::get().to(handlers::geoloc::get_locations))
            ).service(
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
