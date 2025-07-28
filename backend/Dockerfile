FROM rust:1.84.1 as builder

WORKDIR /usr/src/backend

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock ./
COPY diesel.toml ./

# Créer un projet factice pour télécharger les dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copier le vrai code source
COPY src ./src
COPY migrations ./migrations
COPY assets ./assets

# Construire l'application
RUN cargo build --release

# Image de production
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copier l'exécutable depuis l'étape de build
COPY --from=builder /usr/src/backend/target/release/backend .

# Copier les assets
COPY --from=builder /usr/src/backend/assets ./assets

EXPOSE 8080

CMD ["./backend"]
