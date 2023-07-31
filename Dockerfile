# 1. This tells docker to use the Rust official image
FROM rust:1.71 as build

COPY ./ ./

RUN cargo build --release

FROM ubuntu:22.04

RUN yes | unminimize
RUN apt-get update
RUN apt-get upgrade --yes
RUN apt-get install --yes vim less 

RUN adduser crust

RUN mkdir -p /home/crust/.local/bin
RUN mkdir -p /home/crust/.local/share/crust_boot_you

COPY --from=build ./target/release/crust_boot_you /home/crust/.local/bin/crust_boot_you

COPY ./xtask/init_data/data /home/crust/.local/share/crust_boot_you

RUN chown -R crust /home/crust

RUN chmod +x /home/crust/.local/bin/crust_boot_you

ENV PATH "$PATH:~/.local/bin"

