config := "config.json"
mode := "debug"

CARGOFLAGS := if mode == "release" { "--release" } else { "" }
TAILWINDFLAGS := if mode == "release" { "--minify" } else { "--watch" }

run:
    HNPARSER_CONFIG_PATH="{{config}}" cargo run {{CARGOFLAGS}}

build-frontend:
    npx tailwindcss -i views/index.css -o ./views/output.css {{TAILWINDFLAGS}}

build-deb:
    cargo deb --deb-revision="$(date +%s)"
