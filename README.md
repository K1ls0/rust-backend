# Geolocation tracking backend

## Geolocation data storage

### Definition

- lattitude: breite
- longitude: laenge

| client_uuid    | lattitude | longitude | refresh_timestamp (UTC) | notes           |
|----------------|-----------|-----------|-------------------------|-----------------|
| UUID (VARCHAR) | REAL      | REAL      | DATE(INTEGER, unix)     | TEXT            |

### Example
- On timestamp `1606145740`


| client_uuid                          | coordinates (latt, long)  | utc_refresh_timestamp   | notes           |
|--------------------------------------|------------|--------------|-------------------------|-----------------|
| 043457d3-6e20-4c01-988e-e9619b8fdef6 | 52.3918800 | 13.2216100   | 1606145980 (+4mi)       | Stahnsdorf      |
| c8c9cdc6-715e-496c-9b04-7cb98cb43ac9 | 52.4554821 | 13.2953937   | 1606145760 (+20s)       | ZIB             |
|                                      |            |              |                         |                 |

## Goals
- sqlite for storage (No complex database like Postgres is needed)
- https (ssl certificate for the api server)
    - OAuth (basic oauth authorization, also for accessing the data)
- Connection pools (don't close connections if next update is closer than a specific threshhold for better performance (client-wise))
