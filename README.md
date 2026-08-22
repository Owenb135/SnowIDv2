# SnowIDv2 ❄️

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://github.com/ArmanX-Labs/SnowIDv2/actions/workflows/rust.yml/badge.svg)](https://github.com/ArmanX-Labs/SnowIDv2/actions/workflows/rust.yml)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-14%20%7C%2015%20%7C%2016%20%7C%2017-blue)](https://www.postgresql.org/)

High-performance, 64-bit Snowflake-style distributed ID generator for PostgreSQL and Rust.

Generate strictly time-ordered, distributed, 64-bit IDs directly inside your database—**zero application-side ID generation needed**.

---

## Why SnowIDv2?

- **Half the storage of UUIDs**: Only **8 bytes (`BIGINT` / `i64`)** vs 16 bytes for UUIDv4/UUIDv7. Uses 50% less RAM, disk, and cache for indexes and foreign keys.
- **Zero B-Tree Index Fragmentation**: Strictly time-ordered IDs ensure append-only B-tree index inserts in PostgreSQL.
- **Language & Framework Independent**: Works seamlessly with any backend language (Node.js, Python, Go, Java, Rust, C#, PHP) or ORM (Prisma, Drizzle, SQLAlchemy, GORM).
- **Blazing Fast**: Generates over **40,000,000 IDs per second** concurrently (~25 nanoseconds per ID).

---

## 🏗️ Architecture

```mermaid
graph TD
    App["Backend Application (Node.js, Rust, Go, Python)"] -->|INSERT INTO ... RETURNING id| PG["PostgreSQL Database"]
    
    subgraph PostgreSQL Layer
        PG -->|DEFAULT snowid()| NativeExt["Native Rust Extension (snowid_pg)"]
        PG -->|DEFAULT snowid_next(1)| PureSQL["Pure SQL Function (postgres_pure.sql)"]
    end
    
    subgraph Core Logic
        NativeExt --> Core["Core Rust Library (snowid)"]
    end
    
    style App fill:#f9f,stroke:#333,stroke-width:2px
    style PG fill:#69b3a2,stroke:#333,stroke-width:2px
    style NativeExt fill:#ff9999,stroke:#333,stroke-width:2px
    style PureSQL fill:#ffb366,stroke:#333,stroke-width:2px
    style Core fill:#ff6666,stroke:#333,stroke-width:2px
```

---

## 🧩 How it Works (Bit Layout)

SnowIDv2 generates a 64-bit integer (`BIGINT`) composed of three parts, keeping the highest bit `0` so it remains positive:

| Timestamp (41 bits) | Machine ID (10 bits) | Sequence (12 bits) |
| :--- | :--- | :--- |
| Milliseconds since custom epoch | Configurable node/worker ID | Auto-incrementing per millisecond |

- **41 bits for timestamp**: Gives us ~69 years of IDs before rolling over.
- **10 bits for machine ID**: Supports up to 1024 unique database nodes or application workers.
- **12 bits for sequence**: Supports generating up to 4,096 unique IDs per millisecond, per machine.

---

## 🚀 Quick Start (PostgreSQL)

### Option 1: Managed Cloud PostgreSQL (AWS RDS, Supabase, Neon, Railway)
Run the turnkey pure SQL script [`sql/postgres_pure.sql`](sql/postgres_pure.sql) in your database query editor:

```sql
-- 1. Run sql/postgres_pure.sql once to define snowid_next(machine_id)

-- 2. Define your table with DEFAULT snowid_next(1)
CREATE TABLE users (
    id BIGINT PRIMARY KEY DEFAULT snowid_next(1),
    username TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Application inserts directly without generating IDs first:
INSERT INTO users (username) VALUES ('alice') RETURNING id;
```

### Option 2: Self-Hosted PostgreSQL / Docker (Native C/Rust Extension)

#### Using Docker & Docker Compose (Recommended for Local Dev / Containerized Deployments)
We provide a multi-stage `Dockerfile` and `docker-compose.yml` that builds and pre-installs the native Rust extension automatically:

```bash
# Start PostgreSQL 17 with the SnowID native extension pre-installed
docker compose up -d --build

# Connect to the database and test ID generation right away
docker exec -it snowid_postgres psql -U postgres -d snowid_demo
```

Or build directly with Docker:
```bash
docker build --build-arg PG_MAJOR=17 -t snowid-postgres:latest .
docker run -d --name snowid-pg -p 5432:5432 -e POSTGRES_PASSWORD=postgres snowid-postgres:latest
```

Once running inside PostgreSQL, enable and use the extension:

```sql
CREATE EXTENSION IF NOT EXISTS snowid;

CREATE TABLE orders (
    id BIGINT PRIMARY KEY DEFAULT snowid(), -- Or DEFAULT snowid_with_machine(2)
    amount NUMERIC(10, 2) NOT NULL
);

INSERT INTO orders (amount) VALUES (99.99) RETURNING id;
```

---

## 🔍 Decoding IDs in SQL

Inspect when any ID was created and which machine node generated it:

```sql
SELECT * FROM snowid_decode(119842790364971008);
```

---

## ⚡ Performance Benchmark

Run the included benchmark on your machine:

```bash
cargo run --release -p snowid --example benchmark
```

```
1. Single-Threaded Generator (`SnowIdGenerator::generate`):
   - Throughput:          16,446,117 IDs/sec
   - Latency per ID:         60.80 ns/ID

2. Multi-Threaded Concurrent Generation (8 Threads across machines):
   - Throughput:          39,718,988 IDs/sec
   - Latency per ID:         25.18 ns/ID
```

---

## 📦 Project Structure

```
SnowIDv2/
├── Dockerfile               # Multi-stage build for PostgreSQL with SnowID pre-installed
├── docker-compose.yml       # Turnkey Docker Compose configuration
├── docker/
│   └── initdb/              # Auto-initialization scripts when running via Docker
├── snowid/                  # Core pure-Rust Snowflake generator library
├── snowid_pg/               # PostgreSQL Extension wrapper (CREATE EXTENSION snowid;)
├── sql/
│   ├── postgres_pure.sql    # Pure PL/pgSQL function for Cloud/Managed Postgres
│   └── schema_examples.sql  # Turnkey schema & zero-app-generation examples
└── README.md
```

---

## 🤝 Contributing

We love contributions! Whether you're fixing a bug, adding a new language example, or improving the documentation, your help is welcome.

We have created several **[Good First Issues](https://github.com/ArmanX-Labs/SnowIDv2/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)** specifically designed for new open-source contributors. Feel free to pick one up!

Please read our [Contributing Guide](CONTRIBUTING.md) to learn how to set up your environment, run tests, and submit a Pull Request. Don't forget to review our [Code of Conduct](CODE_OF_CONDUCT.md) as well.
