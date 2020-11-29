# SQL Statements to use

### table creation

```sql
CREATE TABLE geoloc_storage(client_uuid VARCHAR(36), lattitude REAL, longitude REAL, utc_refresh_timestamp INTEGER, notes TEXT);
```

### insert new client into table

```sql
INSERT INTO geoloc_storage 
VALUES("{UUID}", {LATT}, {LONG}, {TIME_UTC}, "{NOTES}");
```

### update fields (lattitude, longitude, update-timestamp)

```sql
UPDATE geoloc_storage 
SET lattitude={LATT}, longitude={LONG}, utc_refresh_timestamp={TIME_UTC}
WHERE client_uuid="$UUID"
```

### update fields (lattitude, longitude, update-timestamp,  notes)

```sql
UPDATE geoloc_storage 
SET lattitude={LATT}, longitude={LONG}, utc_refresh_timestamp={TIME_UTC}, notes
WHERE client_uuid="$UUID"
```
