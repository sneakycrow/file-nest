set dotenv-load := true

default:
    just --list

install:
    cargo check
    npm install

build-api:
    cargo build -p api

build-fe:
    npm run build

build-app:
    cargo build -p stream-uploader

build: build-api build-fe

start: start-api
start-api:
    cargo run -p api

dev: dev-api
dev-api:
    bacon run -- api

dev-app:
    just stream-uploader/dev

migrate:
    sqlx migrate run

watch-css:
    npm run watch:tailwind

# This won't dereference them in the database
clean-uploads:
    rm -rf uploads/*

start-queue:
    cargo run -p event-processor
