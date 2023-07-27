# 1. This tells docker to use the Rust official image
FROM rust:1.71 as build

COPY ./ ./

RUN cargo build --release

FROM ubuntu:22.04

RUN adduser crust

RUN mkdir -p /home/crust/.local/bin

COPY --from=build ./target/release/crust_boot_you /home/crust/.local/bin/crust_boot_you

RUN chown crust /home/crust/.local/bin/crust_boot_you

RUN chmod +x /home/crust/.local/bin/crust_boot_you

ENV PATH "$PATH:~/.local/bin"

