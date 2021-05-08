use rusqlite::{ Connection, Result, params, Error };

use uuid::Uuid;

use crate::types::{ GeoLocation, RXGeoData, TXGeoData };
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct SQLDataBase {
    filepath: String,
    conn: Connection,
    table_name: String
}

impl Clone for SQLDataBase {
    fn clone(&self) -> SQLDataBase {
         SQLDataBase::new(&self.filepath, &self.table_name).expect("Error while Cloning the SQL Data base adapter.")
    }
}

impl SQLDataBase {
    pub fn new(filepath: &String, table_name: &String) -> Result<SQLDataBase, Error> {
        Ok(SQLDataBase {
            filepath: filepath.clone(),
            conn: Connection::open(filepath.clone())?,
            table_name: table_name.clone()
        })
    }

    pub fn add_client(&self, uuid: Uuid, loc: &GeoLocation, update_tsp: &NaiveDateTime) -> Result<(), Error> {
        self.add_client_notes(uuid, loc, update_tsp, String::from(""))
    }

    pub fn add_client_notes(
        &self,
        uuid: Uuid,
        loc: &GeoLocation,
        update_tsp: &NaiveDateTime,
        notes:String
        ) -> Result<(), Error> {
        self.conn.execute("
            INSERT INTO ?1
            VALUES (\"?2\", ?3, ?4, ?5, \"?6\");
        ", params![
            self.table_name,
            uuid.to_string(),
            (*loc).lat,
            (*loc).long,
            update_tsp.timestamp(), 
            notes
        ])?;
        Ok(())
    }

    pub fn update_position_notes(
        &self, 
        uuid: Uuid, 
        loc: &GeoLocation, 
        update_tsp: &NaiveDateTime, 
        notes: String
        ) -> Result<(), Error> {
        self.conn.execute("
            UPDATE ?1
            SET lattitude=?2, longitude=?3, utc_refresh_timestamp=?4, notes=?5
            WHERE client_uuid=\"?6\"
        ", params![
            self.table_name,
            loc.lat,
            loc.long,
            update_tsp.timestamp(),
            notes,
            uuid.to_string()
        ])?;
        Ok(())
    }


    pub fn update_position(
        &self, 
        uuid: Uuid, 
        loc: &GeoLocation, 
        update_tsp: &NaiveDateTime
        ) -> Result<(), Error> {
        self.conn.execute("
            UPDATE ?1
            SET lattitude=?2, longitude=?3, utc_refresh_timestamp=?3
            WHERE client_uuid=\"?4\"
        ", params![
            self.table_name,
            loc.lat,
            loc.long,
            update_tsp.timestamp(),
            uuid.to_string()
        ])?;
        Ok(())
    }

    pub fn delete_client(&self, uuid: Uuid) -> Result<(), Error> {
        self.conn.execute("
            DELETE FROM ?1
            WHERE client_uuid=\"?2\"
        ", params![
            self.table_name,
            uuid.to_string()
        ])?;
        Ok(())
    }

    pub fn get_client(&self, uuid: Uuid) -> Result<std::collections::LinkedList<GeoLocation>, Error> {
        let data = self.conn.execute("
            SELECT * FROM ?1
            WHERE client_uuid=\"?2\"
        ", params![
            self.table_name,
            uuid.to_string()
        ])?;
        println!("Got Data: {:?}", data);
        Ok(std::collections::LinkedList::new())
    }
}
