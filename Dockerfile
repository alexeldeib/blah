# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

ARG APP=blah
ENV APP=$APP

ENV USER=root

WORKDIR /usr/src/blah

COPY . .

RUN cargo vendor

RUN cargo build --release

RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM gcr.io/distroless/cc

COPY --from=cargo-build /usr/src/blah/target/release/blah /usr/local/bin/blah

CMD ["/usr/local/bin/blah"]