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

    pub async fn add_location(db: web::Data<SQLDataBase>, payload: web::Json<types::GeoData>) -> Result<impl Responder, Error> {
        let geodat = types::GeoData::from(payload);
        debug!("Adding location");

        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Added location successfully"), 200)))
    }

    pub async fn get_locations(db: web::Data<SQLDataBase>) -> Result<impl Responder, Error> {
        debug!("updating location");
        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Getting locations"), 200)))
    }
    
    pub async fn render_404(req: HttpRequest) -> Result<impl Responder, Error> {
        debug!("Invalid request made (to '{}')", req.uri());

        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("404 Not found"), 404)))
    }

}
