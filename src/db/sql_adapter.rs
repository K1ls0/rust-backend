use std::fmt;

use sqlx::postgres;

use uuid::Uuid;
use chrono::NaiveDateTime;

use crate::types::rest;
use super::models;

use log::{ warn, info, error, debug, trace };


pub mod errors {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub enum ConError {
        DatabaseError(String),
        TodoError,
        InvalidFieldError(String),
    }

    impl From<sqlx::Error> for ConError {
        fn from(i: sqlx::Error) -> Self {
            ConError::DatabaseError(format!("Connection error database: \"{}\"", i))
        }
    }
    impl From<uuid::Error> for ConError {
        fn from(i: uuid::Error) -> Self {
            ConError::DatabaseError(format!("uuid traverse error: \"{}\"", i))
        }
    }

    impl fmt::Display for ConError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ConError::DatabaseError(s) => write!(f, "Connection Error: {}", s),
                ConError::TodoError => write!(f, "Not implemented yet. TODO!"),
                ConError::InvalidFieldError(s) => write!(f, "Invalid Field Error: {}", s),
            }
        }
    }
}

#[derive(Clone)]
pub struct SQLDataBase {
    path: String,
    con_pool: postgres::PgPool,
}

impl fmt::Debug for SQLDataBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SQL Database")
            .field("path", &self.path)
            .finish()
    }
}

impl SQLDataBase {
    pub async fn new(path: &String, max_con: u32) -> Result<SQLDataBase, errors::ConError> {
        Ok(SQLDataBase {
            path: path.clone(),
            con_pool: postgres::PgPoolOptions::new()
                .max_connections(max_con)
                .connect(path.as_str())
                .await
                .map_err(|e| errors::ConError::from(e))?,
        })
    }

    pub async fn add_client(
        &self,
        uuid: Uuid,
        loc: &rest::GeoLocation,
        update_tsp: &NaiveDateTime
        ) -> Result<(), errors::ConError> {
        info!("Adding client to db");
        self.add_client_notes(uuid, loc, update_tsp, String::from("")).await
    }

    pub async fn add_client_notes(
        &self,
        uuid: Uuid,
        loc: &rest::GeoLocation,
        update_tsp: &NaiveDateTime,
        note: String
        ) -> Result<(), errors::ConError> {
        info!("Adding client with notes to db");

        // VALUES({UUID}, {LATT}, {LONG}, {TIME_UTC}, {NOTES});
        sqlx::query!(r#"
            INSERT INTO geoloc_storage ( client_uuid, lattitude, longitude, refresh,  notes )
            VALUES( $1, $2, $3, $4, $5 );
            "#,
            uuid.to_hyphenated().to_string(),
            loc.lat,
            loc.long,
            update_tsp,
            note)
            .fetch_optional(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;
        Ok(())
    }

    pub async fn _update_position_notes(
        &self, 
        uuid: Uuid, 
        loc: &rest::GeoLocation, 
        update_tsp: &NaiveDateTime, 
        note: String
        ) -> Result<(), errors::ConError> {
        info!("updating client position with notes to db");

        sqlx::query!(r#"
            UPDATE geoloc_storage
            SET lattitude=$2, longitude=$3, refresh=$4, notes=$5
            WHERE client_uuid=$1
            "#,
            uuid.to_hyphenated().to_string(),
            loc.lat,
            loc.long,
            update_tsp,
            note)
            .fetch_optional(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;
        Ok(())
    }


    pub async fn update_position(
        &self, 
        uuid: Uuid, 
        loc: &rest::GeoLocation, 
        update_tsp: &NaiveDateTime
        ) -> Result<(), errors::ConError> {
        info!("updating client position to db");

        sqlx::query!(r#"
            UPDATE geoloc_storage
            SET lattitude=$2, longitude=$3, refresh=$4
            WHERE client_uuid=$1
            "#,
            uuid.to_hyphenated().to_string(),
            loc.lat,
            loc.long,
            update_tsp)
            .fetch_optional(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;

        Ok(())
    }

    pub async fn _delete_client(&self, uuid: Uuid) -> Result<(), errors::ConError> {
        info!("deleting client from db");

        sqlx::query!(r#"
            DELETE FROM geoloc_storage
            WHERE client_uuid=$1
            "#,
            uuid.to_hyphenated().to_string())
            .fetch_optional(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;
        Ok(())
    }

    pub async fn get_client(&self, uuid: Uuid) -> Result<rest::GeoData, errors::ConError> {
        info!("get specific client from db");

        let c = sqlx::query_as!(
            models::GeoLocData,
            r#"
            SELECT client_uuid, lattitude, longitude, refresh, notes
            FROM geoloc_storage
            WHERE client_uuid=$1
            "#,
            uuid.to_hyphenated().to_string())
            .fetch_one(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;

        Ok(rest::GeoData { 
            uuid: Uuid::parse_str(&c.client_uuid)?,
            loc: rest::GeoLocation { lat: c.longitude, long: c.longitude },
            refresh_time: c.refresh,
        })
    }

    pub async fn get_all_clients(&self) -> Result<rest::GeoDataList, errors::ConError> {
        info!("get all clients from db");

        let rows = sqlx::query_as!(
            models::GeoLocData,
            r#"
            SELECT client_uuid, lattitude, longitude, refresh, notes
            FROM geoloc_storage
            "#)
            .fetch_all(&self.con_pool)
            .await
            .map_err(|e| errors::ConError::from(e))?;

        Ok(rest::GeoDataList {
            items: rows.iter().map(|s|
                rest::GeoData {
                    uuid: uuid::Uuid::parse_str(&s.client_uuid).unwrap(),
                    loc: rest::GeoLocation { lat: s.lattitude, long: s.longitude },
                    refresh_time: s.refresh,
                }).collect(),
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn setup_database(test_data: Option<&[rest::GeoData]>) -> String {
        //let path = "test_sqlite_adapter.db";

        //let _ = fs::remove_file(path);
        /*let con = Connection::open(path.clone());
        match con {
            Ok(con) => {
                let _ = con.execute(format!("DROP TABLE {}", table).as_str(), params![]);
                let res = con.execute(format!("CREATE TABLE {}(
                    client_uuid VARCHAR(36) NOT NULL PRIMARY KEY, 
                    lattitude REAL, 
                    longitude REAL, 
                    utc_refresh_timestamp INTEGER, 
                    notes TEXT
                );", table).as_str(), params![]);
                match res {
                    Ok(_) => {},
                    Err(e) => assert!(false, format!("[setup_database] Unexpected error while creating new test table manually ({:?})", e))
                }


                if let Some(data) = test_data {
                    for c_dat in data {
                        let res = con.execute(format!("
                            INSERT INTO {} (client_uuid, lattitude, longitude, utc_refresh_timestamp, notes)
                            VALUES (?1, ?2, ?3, ?4, ?5);
                        ", table).as_str(), params![
                            c_dat.uuid.to_string(),
                            c_dat.loc.lat,
                            c_dat.loc.long,
                            c_dat.refresh_time.timestamp(),
                            "test",
                        ]);
                        match res {
                            Ok(_) => {}
                            Err(e) => {assert!(false, format!("Unexpected Error while inserting data (Got {:?})", e));}
                        }
                    }
                }


                return (String::from(path), String::from(table));
            },
            Err(_) => {assert!(false, "[setup_database] Unexpected error while opening test database itself.");}
        }*/
        String::from("")
    }

    #[actix_rt::test]
    async fn test_new_database() {
        let params = [
            [String::from("a"), String::from("b")],
            [String::from("test1"), String::from("test2")],
            [String::from("bc"), String::from("a")],
        ];

        for _c_set in &params {
            /*let new = SQLDataBase::new(&c_set[0], &c_set[1]);
            
            match new {
                Ok(n) => {
                    assert_eq!(n.filepath, c_set[0]);
                    assert_eq!(n.table_name, c_set[1]);
                }
                Err(_) => {assert!(false, "False error while creating database object.");}
            }

            match fs::remove_file(&c_set[0]) {
                Ok(()) => {},
                Err(_) => assert!(false, "No database file created."),
            }*/
        }
    }

    #[actix_rt::test]
    async fn test_get_client() {
        let test_table_data = [
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fdef6").unwrap(),
                loc: rest::GeoLocation { lat: 0.0, long: 0.0 },
                refresh_time: NaiveDateTime::from_timestamp(1614580845, 0)
            },
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fde00").unwrap(),
                loc: rest::GeoLocation { lat: 3.14159265, long: 10.01 },
                refresh_time: NaiveDateTime::from_timestamp(0, 0),
            }
        ];
        let _path = setup_database(Some(&test_table_data));
        /*
        match SQLDataBase::new(&path, 2) {
            Ok(db) => {
                for c_t_data in &test_table_data {
                    let c_res = db.get_client(c_t_data.uuid);
                    match c_res {
                        Ok(res) => {
                            assert_eq!(res.uuid, c_t_data.uuid);
                            assert_eq!(res.loc, c_t_data.loc);
                            assert_eq!(res.refresh_time, c_t_data.refresh_time);
                        }
                        Err(e) => {assert!(false, format!("Unexpected error while creating Database ({:?})", e));}
                    }
                }
            }
            Err(e) => {assert!(false, format!("Unexpected Error while creating SQL Database ({:?})", e));}
        }
        */
    }

    #[actix_rt::test]
    async fn test_add_client() {
        let _params = [
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fdef6").unwrap(),
                loc: rest::GeoLocation { lat: 0.0, long: 0.0 },
                refresh_time: NaiveDateTime::from_timestamp(1614580845, 0)
            },
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fde00").unwrap(),
                loc: rest::GeoLocation { lat: 3.14159265, long: 10.01 },
                refresh_time: NaiveDateTime::from_timestamp(0, 0)
            },
        ];
        let path = setup_database(None);
        let _db = SQLDataBase::new(&path, 2);

        /*match db {
            Ok(db) => {
                for c in params.iter() {
                    match db.add_client(c.uuid, &c.loc, &c.refresh_time) {
                        Ok(()) => {}
                        Err(_) => assert!(false)
                    }
                    match db.get_client(c.uuid) {
                        Ok(c) => {
                            assert_eq!(c.uuid, c.uuid);
                            assert_eq!(c.loc, c.loc);
                            assert_eq!(c.refresh_time, c.refresh_time);
                        }
                        Err(_) => {assert!(false);}
                    }
                }

                match db.add_client(params[0].uuid, &params[0].loc, &params[0].refresh_time) {
                    Ok(()) => assert!(false, "Client should not be able to being added twice, returned successfully."),
                    Err(_) => {},
                }
            }
            Err(_) => assert!(false)
        }*/
    }

    #[actix_rt::test]
    async fn test_update_position() {
        let params = [
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fdef6").unwrap(),
                loc: rest::GeoLocation { lat: 0.0, long: 0.0 },
                refresh_time: NaiveDateTime::from_timestamp(1614580845, 0)
            },
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fde00").unwrap(),
                loc: rest::GeoLocation { lat: 3.14159265, long: 10.01 },
                refresh_time: NaiveDateTime::from_timestamp(0, 0)
            },
        ];

        let path = setup_database(Some(&params));
        let _db = SQLDataBase::new(&path, 2);
        /*match db {
            Ok(db) => {
                match db.update_position(
                    params[0].uuid,
                    &rest::GeoLocation { lat: 0.1258, long: 10.427 },
                    &chrono::NaiveDateTime::from_timestamp(10, 0)) {
                    Ok(()) => {},
                    Err(e) => assert!(false, format!("Error while updating position: {:?}", e))
                }

                match db.get_client(params[0].uuid) {
                    Ok(c) => {
                        assert_eq!(c.uuid, params[0].uuid);
                        assert_eq!(c.loc, rest::GeoLocation { lat: 0.1258, long: 10.427 });
                        assert_eq!(c.refresh_time, chrono::NaiveDateTime::from_timestamp(10, 0));
                    },
                    Err(e) => assert!(false, format!("Error while getting client after update: {:?}", e))
                }
                match db.get_client(params[1].uuid) {
                    Ok(c) => {
                        assert_eq!(c.uuid, params[1].uuid);
                        assert_eq!(c.loc, params[1].loc);
                        assert_eq!(c.refresh_time, params[1].refresh_time);
                    }
                    Err(e) => assert!(false, format!("Error while getting untouched client: {:?}", e))
                }

                match db.update_position_notes(
                    params[0].uuid,
                    &params[0].loc,
                    &params[0].refresh_time, 
                    String::from("This is a test note")) {
                    Ok(()) => {},
                    Err(e) => assert!(false, format!("Error while updating position: {:?}", e))
                }
                match db.get_client(params[0].uuid) {
                    Ok(c) => {
                        assert_eq!(c.uuid, params[0].uuid);
                        assert_eq!(&c.loc, &params[0].loc);
                        assert_eq!(&c.refresh_time, &params[0].refresh_time);
                    },
                    Err(e) => assert!(false, format!("Error while updating position: {:?}", e))
                }

            }
            Err(_) => assert!(false)
        }*/
    }
    
    #[actix_rt::test]
    async fn test_delete_client() {
        let test_table_data = [
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fdef6").unwrap(),
                loc: rest::GeoLocation { lat: 0.0, long: 0.0 },
                refresh_time: NaiveDateTime::from_timestamp(1614580845, 0)
            },
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fde00").unwrap(),
                loc: rest::GeoLocation { lat: 3.14159265, long: 10.01 },
                refresh_time: NaiveDateTime::from_timestamp(0, 0),
            }
        ];
        let _path = setup_database(Some(&test_table_data));

        /*match SQLDataBase::new(&path, &table) {
            Ok(db) => {
                for c_t_data in &test_table_data {
                    match db.delete_client(c_t_data.uuid) {
                        Ok(()) => {},
                        Err(e) => assert!(false, format!("Unexpected Error while deleting client({:?})", e)),
                    }

                    //Try to delete it again, should not work
                    match db.delete_client(c_t_data.uuid) {
                        Ok(()) => assert!(false, "One should not be able to delete entries twice."),
                        Err(_) => {},
                    }
                }
            }
            Err(e) => {assert!(false, format!("Unexpected Error while creating SQL Database ({:?})", e));}
        }*/
    }

    
    fn print_all_rows(_path: &String) {
        todo!()

        /*let con = Connection::open(path.clone());
        match con {
            Ok(con) => {
                let mut statement = con.prepare(format!("
                SELECT * FROM {};
                ", table).as_str())
                    .unwrap();


                let res = statement.query_map(params![], |r| {
                    if columns.len() > 0 {
                        r.column_names().iter().for_each(|c| columns.push(c.clone()));
                    }
                    Ok(())
                }).unwrap();
                println!("Res length: {}", res.fold(0, |sum, _| sum+1));

            }
            Err(e) => println!("\"{}:?\" occured", e)
        }*/
    }

    #[test]
    fn test_get_all_clients() {
        let params = [
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fdef6").unwrap(),
                loc: rest::GeoLocation { lat: 0.0, long: 0.0 },
                refresh_time: NaiveDateTime::from_timestamp(1614580845, 0)
            },
            rest::GeoData {
                uuid: Uuid::parse_str("043457d3-6e20-4c01-988e-e9619b8fde00").unwrap(),
                loc: rest::GeoLocation { lat: 3.14159265, long: 10.01 },
                refresh_time: NaiveDateTime::from_timestamp(0, 0)
            },
        ];
        let path = setup_database(Some(&params));
        let _db = SQLDataBase::new(&path, 2);

        print_all_rows(&path);
    }
}
