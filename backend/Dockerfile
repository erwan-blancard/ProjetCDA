FROM rust:1.84.1

WORKDIR /usr/src/backend

# required for polodb
RUN apt-get update
RUN apt-get install clang -y

COPY ./Cargo.toml ./Cargo.toml
COPY ./diesel.toml ./diesel.toml

COPY ./migrations ./migrations
COPY ./src ./src

RUN cargo install --path .

ENV DATABASE_URL=postgres://localhost:5432/randomi
ENV CARDS_FILE_PATH=/usr/src/backend/data/cards.json
ENV BACKEND_DB_PATH=/tmp/randomi-backend-db

EXPOSE 8080

CMD [ "backend" ]
