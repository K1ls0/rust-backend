#!/bin/sh

echo MODE: $MODE
if [  "$MODE" == "RUN" ]; then
    echo "Running webserver"
    cargo run
elif [ "$MODE" == "DEBUG" ]; then
    echo "Running compilation debugging"
    cd src && watchmedo shell-command --patterns="*.rs" --recursive --command="cargo build"
else
    echo "ENV 'MODE' needs to be set to either DEBUG or RUN, stopping here"
fi
