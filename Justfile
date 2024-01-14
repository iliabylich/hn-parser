run:
    HNPARSER_CONFIG_PATH=config.json cargo watch -x run

tailwind-build:
    npx tailwindcss -i templates/index.css -o ./templates/output.css --minify

tailwind-watch:
    npx tailwindcss -i templates/index.css -o ./templates/output.css --watch

build-deb:
    cargo deb --deb-revision="$(date +%s)"
