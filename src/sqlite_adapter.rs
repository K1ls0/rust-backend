use rusqlite::{ Connection, Result, params, Error };
use rusqlite::NO_PARAMS;

use uuid::Uuid;

use crate::types::{ GeoData, GeoLocation };

#[derive(Debug)]
struct SQLDataBase {
    conn: Connection,
}

impl SQLDataBase {
    pub fn new(file: String) -> Result<SQLDataBase, Error> {
        Ok(SQLDataBase {
            conn: Connection::open(file)?,
        })
    }

    pub fn add_client(self, uuid: Uuid) -> Result<SQLDataBase, Error> {
        self.conn.execute("

            ", params![],)?;
    }

    pub fn update_position(loc: GeoLocation) {

    }
}
