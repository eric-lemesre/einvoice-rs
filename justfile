set shell := ["bash", "-cu"]
set dotenv-load := true

# URL de la base utilisée par sqlx-cli
db_url := env_var_or_default("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/einvoice")

# Liste les recettes disponibles
default:
    @just --list

# --- Exécution ----------------------------------------------------------------

# Démarre l'API REST
run-api:
    cargo run -p einvoice-api

# Démarre le frontend web
run-web:
    cargo run -p einvoice-web

# --- Build --------------------------------------------------------------------

# Compile tout le workspace
build:
    cargo build --workspace

# Compile en mode release
build-release:
    cargo build --workspace --release

# Nettoie les artefacts
clean:
    cargo clean

# --- Qualité ------------------------------------------------------------------

# Formate le code
fmt:
    cargo fmt --all

# Vérifie le format sans modifier
fmt-check:
    cargo fmt --all -- --check

# Lint avec clippy (warnings = errors)
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Exécute les tests
test:
    cargo test --workspace

# Suite complète: fmt + lint + test
check: fmt-check lint test

# Vérifie les licences, advisories et bans (cargo-deny requis)
deny:
    cargo deny check

# Rapport de couverture HTML (cargo-llvm-cov requis)
coverage:
    cargo llvm-cov --workspace --html --open

# Rapport de couverture LCOV (pour CI / Codecov)
coverage-lcov:
    cargo llvm-cov --workspace --lcov --output-path lcov.info

# Nettoie les artefacts de couverture
coverage-clean:
    cargo llvm-cov clean --workspace

# Tests par mutation sur les crates métier (cargo-mutants requis)
mutants:
    cargo mutants --package einvoice-core --package einvoice-facturx --package einvoice-ubl --timeout 120 --jobs 2

# Configure git pour utiliser les hooks du dépôt
setup-hooks:
    git config core.hooksPath .githooks

# --- Base de données ----------------------------------------------------------

# Lance une instance PostgreSQL locale via Docker
db-up:
    docker run -d --name einvoice-pg \
        -e POSTGRES_USER=postgres \
        -e POSTGRES_PASSWORD=postgres \
        -e POSTGRES_DB=einvoice \
        -p 5432:5432 \
        postgres:16

# Arrête et supprime l'instance PostgreSQL locale
db-down:
    docker rm -f einvoice-pg

# Applique les migrations (sqlx-cli requis)
db-migrate:
    sqlx migrate run --database-url {{db_url}}

# Revient sur la dernière migration
db-revert:
    sqlx migrate revert --database-url {{db_url}}

# Installe sqlx-cli avec les features requises
install-sqlx:
    cargo install sqlx-cli --version ^0.8 --no-default-features --features rustls,postgres --locked
