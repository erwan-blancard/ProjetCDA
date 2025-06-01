FROM rust:1.84.1

WORKDIR /usr/src/backend

COPY ./src ./src
COPY ./migrations ./migrations

# grab assets folder from build context "assets"
COPY --from=assets ./assets/cards.json ./assets/cards.json

COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

RUN cargo install --path .

ENV DATABASE_URL postgres://localhost:3307/randomi

EXPOSE 8080

CMD [ "backend" ]
