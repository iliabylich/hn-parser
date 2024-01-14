run:
    HNPARSER_CONFIG_PATH=config.json cargo run

tailwind-build:
    npx tailwindcss -i views/index.css -o ./views/output.css --minify

tailwind-watch:
    npx tailwindcss -i views/index.css -o ./views/output.css --watch

build-deb:
    cargo deb --deb-revision="$(date +%s)"
