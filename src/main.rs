/*#[macro_use]
extern crate log;
extern crate env_logger;
extern crate serde;
extern crate serde_json;
extern crate chrono;

extern crate rusqlite;*/

mod db;
mod handlers;
mod types;
mod util;


//use actix_web::{web, App, HttpResponse, Responder};
use std::env;
use std::io::Write;
use chrono::Local;
use dotenv;

use log::{Level, debug, error, info, warn};
use env_logger::{
    fmt::Color,
    Env,
};

use actix_web::{
    web, 
    App, 
    HttpServer,
    middleware::Logger,
};

//use db::sql_worker::SQLActor;
use db::sql_adapter::SQLDataBase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Setup logging
    env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
        .format(|buf, rec| {
            let lvl = rec.level();
            let mut lvl_style = buf.style();

            match lvl {
                Level::Trace => lvl_style.set_color(Color::White),
                Level::Debug => lvl_style.set_color(Color::Blue),
                Level::Info  => lvl_style.set_color(Color::Green),
                Level::Warn  => lvl_style.set_color(Color::Yellow),
                Level::Error => lvl_style.set_color(Color::Red),
            };
            
            writeln!(buf,
                "{}: [{:>6}]: [{:>10}] \t {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                lvl_style.value(rec.level()),
                rec.target(),
                rec.args(),
            )
        })
        .init();

    // setup sql database connection
    let sql_db = SQLDataBase::new(
                //&String::from("./db/geoloc-data.db"),
                &String::from(env::var("DATABASE_URL")
                    .expect("Please provide the database url with username and password using the 'DATABASE_URL' environment variable")),
                3
            )
        .await
        .expect("Error while connecting to database adapter. stopping...");


    HttpServer::new(move || {
        debug!("Initializing Http Server");

        App::new()
            .wrap(Logger::default()) // Enable logging
            .data(sql_db.clone())
            // verbose api
            .service(
                web::resource("/geoloc/publishLocation")
                .route(web::post().to(handlers::geoloc::update_client))
            )
            .service(
                web::resource("/geoloc/locations")
                .route(web::get().to(handlers::geoloc::get_clients))
            )
            .service(
                web::resource("/geoloc/location/{uuid}")
                .route(web::get().to(handlers::geoloc::get_client))
            )
            // Standard api
            .service(
                web::resource("/geoloc")
                .route(web::get().to(handlers::geoloc::get_clients))
                .route(web::post().to(handlers::geoloc::update_client))
            )
            .service(
                web::resource("/geoloc/{uuid}")
                .route(web::get().to(handlers::geoloc::get_client))
            )

            .default_service(
                web::route().to(handlers::geoloc::render_404)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
