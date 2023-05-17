SHELL := /bin/bash

APP_NAME = rust-rest-api
APP_NAME := $(APP_NAME)

.PHONY: help 
.INTERMEDIATE: 

help:
	@grep -E '^[1-9a-zA-Z_-]+:.*?## .*$$|(^#--)' $(MAKEFILE_LIST) \
	| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m %-43s\033[0m %s\n", $$1, $$2}' \
	| sed -e 's/\[32m #-- /[33m/'

up-dev:
	docker-compose up -d
	./wait-for-it.sh localhost:5429
	
clean:
	docker-compose down
	docker system prune -f --volumes

migrate-up:
	diesel migration run

migrate-down:
	diesel generate redo

start-dev:
	cargo watch -q -c -w src/ -x run

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add rand_core --features "std"
	cargo add diesel --features "postgres r2d2 chrono"
	cargo add anyhow
	cargo install cargo-watch
	cargo install sqlx-cli
	cargo install diesel_cli --no-default-features --features "postgres"