db_url="localhost:80"

all:
	cargo build
run: 
	cargo run

migrate:
	export DATABASE_URL=${db_url} diesel migration redo
