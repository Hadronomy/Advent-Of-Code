_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test year day:
    cargo nextest run -p aoc{{year}}-day-{{day}}

run year day part *release:
    @if [ "{{release}}" = "release" ]; then \
        cargo run -p aoc{{year}}-day-{{day}} --bin part{{part}} --release; \
    else \
        cargo run -p aoc{{year}}-day-{{day}} --bin part{{part}}; \
    fi

bench year day:
    cargo bench -p aoc{{year}}-day-{{day}}

[no-cd]
create year day:
    @if [ ! -d {{source_directory()}}/{{year}} ]; then \
        mkdir {{source_directory()}}/{{year}}; \
    fi
    @cd {{source_directory()}}/{{year}}; \
    cargo generate --path {{source_directory()}}/daily-template --name day-{{day}} --define year={{year}} --define day={{day}}
    @if ! {{source_directory()}}/scripts/get-aoc-input.py {{year}} day-{{day}} --cwd {{source_directory()}} --timeout 60; then \
        echo "Failed to get input for day-{{day}} of year {{year}}"; \
        echo "Cleaning up..."; \
        rm -rf {{source_directory()}}/{{year}}/day-{{day}}; \
        exit 1; \
    fi

[private]
cleanup year day:
    @rm -rf {{source_directory()}}/{{year}}/day-{{day}}
