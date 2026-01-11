# 🏠 Property Backend

Backend service for managing real estate listings and tracking their price history over time.

## ✨ Features
- Property creation and management
- Price history via immutable snapshots
- REST API built with Axum
- PostgreSQL + SeaORM
- Modular architecture (entities, input models, repositories)

---

## 🧱 High-level Architecture

```mermaid
flowchart LR
    API[HTTP API / Axum]
    IN[Input Models]
    REPO[Repositories]
    ENT[Entities - SeaORM]
    DB[(PostgreSQL)]

    API --> IN
    IN --> REPO
    REPO --> ENT
    ENT --> DB

```

## Database Schema

```mermaid
erDiagram
    PROPERTY_LISTING {
        UUID id PK
        STRING external_id
        STRING title
        STRING description
        INT sqm
        STRING city
        STRING municipality
        STRING market_type
        DATE listed_at
    }

    PRICE_SNAPSHOT {
        UUID id PK
        UUID property_id FK
        INT price_amount
        STRING currency
        DATETIME snapshot_at
    }

    PROPERTY_LISTING ||--o{ PRICE_SNAPSHOT : "has"
```

## Running Locally

Set .env file:

- DB_URL=postgres://user:password@localhost:5432/property_tracker
- SKIP_MIGRATIONS=false
- APP_HOST=0.0.0.0
- APP_PORT=8080


## Run migrations:

cargo run -p migration -- up


## Start backend server:

cargo run


Server will listen on APP_HOST:APP_PORT.
