# Rust

RUST_LOG=actix_web=debug cargo run

# Curl

### POST /geoloc

```bash
curl -X POST -H "Content-Type: application/json" -d '\
{ \
    "uuid": "043457d3-6e20-4c01-988e-e9619b8fdef6", \
    "loc": { \
        "lat": 52.3918800, \
        "long": 13.2216100 \
    }, \
    "refresh_time": "2020-11-23T15:36:00" \
}' localhost:8080/geoloc
```
```bash
curl -s -X POST -H "Content-Type: application/json" -d '{"uuid": "043457d3-6e20-4c01-988e-e9619b8fdef6","loc": {"lat": 52.3918800,"long": 13.2216100},"refresh_time": "2020-11-23T15:36:00"}' localhost:8080/geoloc | python3 -mjson.tool
```

### GET /geoloc

```bash
curl -s -X GET localhost:8080/geoloc | python3 -mjson.tool
```
