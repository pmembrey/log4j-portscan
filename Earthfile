VERSION 0.6
FROM rust:1.57

WORKDIR /log4j-portscan

copy-source:
    COPY . ./

build-dev:
    FROM +copy-source
    RUN cargo build
    SAVE ARTIFACT target/debug/log4j-portscan /release/log4j-portscan-dev AS LOCAL artifacts/log4j-portscan-dev

build:
    FROM +copy-source
    RUN cargo build --release
    SAVE ARTIFACT target/release/log4j-portscan /release/log4j-portscan AS LOCAL artifacts/log4j-portscan
    