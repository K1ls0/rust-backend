FROM alpine:latest

RUN apk upgrade
RUN apk add --no-cache rust cargo sqlite sqlite-dev

RUN apk add --no-cache vim python3 py3-pip

RUN pip3 install --no-cache-dir wheel watchdog argh pyYAML

RUN adduser --uid 6604 --home /geoloc --disabled-password --shell /bin/sh geoloc
USER geoloc

WORKDIR /geoloc

COPY Cargo.toml .
COPY ./entrypoint.sh .
COPY ./watch-filesystem .

# COPY ./src ./src

# RUN cargo build

CMD ["./entrypoint.sh"]
