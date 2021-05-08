CREATE TABLE geoloc_storage(
    client_uuid VARCHAR(36) NOT NULL PRIMARY KEY, 
    lattitude REAL NOT NULL,
    longitude REAL NOT NULL, 
    refresh TIMESTAMP NOT NULL, 
    notes TEXT
    );
