_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test day:
    cargo nextest run --locked -p {{day}}

[no-cd]
create year day:
    @if [ ! -d {{source_directory()}}/{{year}} ]; then \
        mkdir {{source_directory()}}/{{year}}; \
    fi
    @cd {{source_directory()}}/{{year}}; \
    cargo generate --path {{source_directory()}}/daily-template --name day-{{day}} --define year={{year}} --define day={{day}}