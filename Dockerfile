FROM rust as builder

RUN rustup component add rustfmt
WORKDIR /usr/src/channels
COPY . .
RUN cargo install --path .

# Build the actual container
FROM scratch
LABEL maintainer="Cian Butler<butlerx@notthe.cloud>"

COPY --from=builder /usr/local/cargo/bin/channels /usr/local/bin/channels
ADD configs/config.toml /etc/channels/config.toml
RUN /usr/local/bin/channels

EXPOSE 9001
ENTRYPOINT ["/usr/local/bin/channels"]
CMD ["-vv"]
