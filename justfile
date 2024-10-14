set dotenv-load := true

default:
    just --list

install:
    cargo check
    npm install

build:
    cargo build --release
    npm run build

start:
    cargo run -p api

dev:
    bacon ex -- api

watch-css:
    npm run watch:tailwind

# This won't dereference them in the database
clean-uploads:
    rm -rf uploads/*

start-queue:
    cargo run -p event-processor
