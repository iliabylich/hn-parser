run:
    cargo watch -x run

tailwind-build:
    npx tailwindcss -i templates/index.css -o ./templates/output.css --minify

tailwind-watch:
    npx tailwindcss -i templates/index.css -o ./templates/output.css --watch

build-deb:
    cargo build --target=x86_64-unknown-linux-musl --release
    strip target/x86_64-unknown-linux-musl/release/hnparser
    cargo deb --deb-revision="$(date +%s)" --target=x86_64-unknown-linux-musl
