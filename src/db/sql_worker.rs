use actix::prelude::*;

use super::sql_adapter::SQLDataBase;
use crate::types::{ rest, msg };

use super::sql_adapter::errors;


#[derive(Debug, Clone)]
pub struct SQLActor {
    pub dat: SQLDataBase,
}

impl Actor for SQLActor {
    type Context = Context<Self>;
}

impl Handler<msg::AddLocation> for SQLActor {
    type Result = AtomicResponse<Self, Result<(), errors::ConnectionError>>;

    fn handle(&mut self, msg: msg::AddLocation, ctx: &mut Context<Self>) -> Self::Result {
        AtomicResponse::new(Box::pin(
            async {
                let msg = msg.0;
                self.dat.add_client(msg.uuid, &msg.loc, &msg.refresh_time)
            }
            .into_actor(self)
            .map(|res, _act, _ctx| {
                res
            }),
        ))
    }
}



impl Handler<msg::GetLocation> for SQLActor {
    type Result = ResponseFuture<Result<rest::GeoData, errors::ConnectionError>>;

    fn handle(&mut self, msg: msg::GetLocation, _ctx: &mut Context<Self>) -> Self::Result {
        match self.dat.get_client(msg.0) {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }
}


impl Handler<msg::GetAllLocations> for SQLActor {
    type Result = Result<rest::GeoDataList, errors::ConnectionError>;

    fn handle(&mut self, _msg: msg::GetAllLocations, _ctx: &mut Context<Self>) -> Self::Result {
        match self.dat.get_all_clients() {
            Ok(s) => Ok(s),
            Err(e) => Err(e),
        }
    }
}
