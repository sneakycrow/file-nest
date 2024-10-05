set dotenv-load := true

start:
    cargo run

watch-css:
    npm run watch:tailwind

dev:
    cargo watch --ignore 'assets/*' -x run

# This won't dereference them in the database
clean-uploads:
    rm -rf uploads/*
