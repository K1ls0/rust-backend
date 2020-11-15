#[macro_use]
extern crate log;

mod handlers;

use actix_web::{
    web, 
    App, 
    HttpServer,
    http::StatusCode,
    middleware::Logger,
    middleware::errhandlers::{
        ErrorHandlers,
    }
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    //let cfg_file = File::open("./service_config.yml")?;


    HttpServer::new(move || {
        debug!("Initializing Http Server");
        App::new()
            .wrap(Logger::default()) // Enable logging
            .wrap(ErrorHandlers::new()
                .handler(StatusCode::NOT_FOUND, handlers::geoloc::return_not_found))
            .service(
                web::resource("/geoloc/publishLocation")
                .route(web::put().to(handlers::geoloc::add_location))
            )
            .service(
                web::resource("/geoloc/locations")
                .route(web::get().to(handlers::geoloc::get_locations))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
