FROM scratch
EXPOSE 9000
COPY ./target/x86_64-unknown-linux-musl/release/rust-service /usr/local/bin/
USER 1000
ENTRYPOINT ["rust-service"]
