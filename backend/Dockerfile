FROM rust:1.84.1

WORKDIR /usr/src/backend

RUN mkdir src
# create empty main.rs file
RUN echo "fn main() {}" > src/main.rs
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
# prefetch dependencies (in case of offline rebuild if no dependencies are changed)
RUN cargo fetch --locked

COPY ./src ./src
COPY ./migrations ./migrations

# grab assets folder from build context "assets"
COPY --from=assets ./assets/cards.json ./assets/cards.json

COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

RUN cargo install --bin backend --locked --path .

ENV DATABASE_URL postgres://localhost:3307/randomi
ENV BACKEND_URL http://localhost:8080

EXPOSE 8080

CMD [ "backend" ]
