pub mod geoloc {
    use actix_web::{
        HttpRequest,
        HttpResponse,
        Responder,
        Error,
    };

    use crate::types;

    pub async fn add_location(_req: HttpRequest) -> Result<impl Responder, Error> {
        println!("addLocation called!");
        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Added location successfully"), 200)))
    }

    pub async fn get_locations(_req: HttpRequest) -> Result<impl Responder, Error> {
        println!("getLocations called!");
        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("Getting locations"), 200)))
    }
    
    pub async fn render_404(_req: HttpRequest) -> Result<impl Responder, Error> {

        Ok(HttpResponse::Ok().json(types::BareStatus::new(String::from("404 Not found"), 404)))
    }

}
