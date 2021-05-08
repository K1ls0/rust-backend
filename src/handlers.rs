pub mod geoloc {
    use actix_web::{
        web,
        HttpRequest,
        HttpResponse,
    };

    use log::{ info, warn };

    use crate::types::rest;
    use crate::db::sql_adapter;
    
    fn internal_err(s: String) -> HttpResponse {
        HttpResponse::InternalServerError()
            .json(rest::BareStatus::new(s, 500))
    }

    pub async fn update_client(db: web::Data<sql_adapter::SQLDataBase>, payload: web::Json<rest::GeoData>) -> HttpResponse {
        info!("Adding location {:?}", payload);

        match db.update_position(payload.uuid, &payload.loc, &payload.refresh_time).await {
            Ok(a) => {
                info!("Updated client successfully!");
                HttpResponse::Ok()
                    .json(rest::BareStatus::new(format!("updated client successfully: \"{:?}\"", a), 200))
            }
            Err(e_upd) => match db.add_client(payload.uuid, &payload.loc, &payload.refresh_time).await {
                Ok(a) => {
                    info!("Added client successfully!");
                    HttpResponse::Ok()
                        .json(rest::BareStatus::new(format!("Added client successfully: \"{:?}\"", a), 200))
                }
                Err(e_add) => {
                    warn!("Could not update nor add the new client:");
                    warn!("update: {:?}", e_upd);
                    warn!("add: {:?}", e_add);

                    internal_err(format!("Could not update nor add the new client: {}; {}", e_upd, e_add))
                }
            }
        }
    }

    pub async fn get_clients(db: web::Data<sql_adapter::SQLDataBase>) -> HttpResponse {
        info!("getting all locations...");
        
        match db.get_all_clients().await {
            Ok(a) => {
                info!("Successfully got locations from db!");
                HttpResponse::Ok()
                    .json(a)
            }
            Err(e) => {
                warn!("{}", e);
                internal_err(format!("{}", e))
            }
        }
    }


    pub async fn get_client(db: web::Data<sql_adapter::SQLDataBase>, pathinfo: web::Path<rest::PathUuid>) -> HttpResponse {
        info!("Getting location from user {}", pathinfo.uuid);
        match db.get_client(pathinfo.uuid).await {
            Ok(a) => {
                HttpResponse::Ok()
                    .json(rest::GeoReturnData::new(
                        a.uuid,
                        a.loc,
                        a.refresh_time,
                    ))
            }
            Err(e) => {
                warn!("{}", e);
                internal_err(format!("{}", e))
            }
        }
    }

    pub async fn render_404(req: HttpRequest) -> HttpResponse {
        info!("Invalid request made (to '{}')", req.uri());

        HttpResponse::Ok().json(rest::BareStatus::new(String::from("404 Not found"), 404))
    }

}
