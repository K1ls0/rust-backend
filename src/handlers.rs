pub mod geoloc {
    use std::sync::Mutex;

    use actix_web::{
        web,
        HttpRequest,
        HttpResponse,
        Responder,
        Error,
    };

    use crate::types;
    use crate::sqlite_adapter::SQLDataBase;

    pub async fn add_location(db: web::Data<types::AnonymGeoLocation>, payload: web::Json<types::RXGeoData>) -> Result<impl Responder, Error> {
        let geodat: types::RXGeoData = payload.into_inner();

        warn!("Adding location");
        debug!("db: {:?}", db);
        debug!("{:?}", geodat);

        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Added location successfully"), 200)))
    }

    pub async fn get_locations(db: web::Data<types::AnonymGeoLocation>) -> Result<impl Responder, Error> {
        warn!("updating location");
        debug!("db: {:?}", db);
        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Getting locations"), 200)))
    }
    
    pub async fn render_404(req: HttpRequest) -> Result<impl Responder, Error> {
        debug!("Invalid request made (to '{}')", req.uri());

        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("404 Not found"), 404)))
    }

}
