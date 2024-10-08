set dotenv-load := true

install:
    cargo check
    npm install

build:
    cargo build --release
    npm run build

start:
    cargo run -p api

watch-css:
    npm run watch:tailwind

dev:
    cargo watch --ignore 'assets/*' -x run

# This won't dereference them in the database
clean-uploads:
    rm -rf uploads/*

start-queue:
    cargo run -p video-processor
