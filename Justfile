_default:
    @just --list

check:
    cargo clippy --locked
    cargo fmt -- --check

fix:
    cargo clippy --fix --locked -- -D warnings

test year day:
    cargo nextest run -p aoc{{year}}-day-{{day}}

run year day part:
    cargo run -p aoc{{year}}-day-{{day}} --bin part{{part}} 

bench year day:
    cargo bench -p aoc{{year}}-day-{{day}}

[no-cd]
create year day:
    @if [ ! -d {{source_directory()}}/{{year}} ]; then \
        mkdir {{source_directory()}}/{{year}}; \
    fi
    @cd {{source_directory()}}/{{year}}; \
    cargo generate --path {{source_directory()}}/daily-template --name day-{{day}} --define year={{year}} --define day={{day}}
    {{source_directory()}}/scripts/get-aoc-input.rs --year {{year}} --day day-{{day}} -c {{source_directory()}}
