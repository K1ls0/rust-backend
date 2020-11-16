pub mod geoloc {
        use actix_web::{ http::StatusCode, HttpRequest, HttpResponse, Responder, dev};

    pub async fn add_location(req: HttpRequest) -> impl Responder {
        println!("addLocation called!");
        HttpResponse::Ok()
    }

    pub async fn get_locations(req: HttpRequest) -> impl Responder {
        println!("getLocations called!");
        HttpResponse::Ok()
    }

    pub async fn return_not_found(r: dev::ServiceResponse) -> impl Responder {
        HttpResponse::new(StatusCode::NOT_FOUND)
    }
}
