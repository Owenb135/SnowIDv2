# SnowIDv2 ❄️

High-performance, 64-bit Snowflake-style distributed ID generator for PostgreSQL and Rust.

Generate strictly time-ordered, distributed, 64-bit IDs directly inside your database—**zero application-side ID generation needed**.

---

## Why SnowIDv2?

- **Half the storage of UUIDs**: Only **8 bytes (`BIGINT` / `i64`)** vs 16 bytes for UUIDv4/UUIDv7. Uses 50% less RAM, disk, and cache for indexes and foreign keys.
- **Zero B-Tree Index Fragmentation**: Strictly time-ordered IDs ensure append-only B-tree index inserts in PostgreSQL.
- **Language & Framework Independent**: Works seamlessly with any backend language (Node.js, Python, Go, Java, Rust, C#, PHP) or ORM (Prisma, Drizzle, SQLAlchemy, GORM).
- **Blazing Fast**: Generates over **40,000,000 IDs per second** concurrently (~25 nanoseconds per ID).

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
Install the PostgreSQL extension:

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
├── snowid/                  # Core pure-Rust Snowflake generator library
├── snowid_pg/               # PostgreSQL Extension wrapper (CREATE EXTENSION snowid;)
├── sql/
│   ├── postgres_pure.sql    # Pure PL/pgSQL function for Cloud/Managed Postgres
│   └── schema_examples.sql  # Turnkey schema & zero-app-generation examples
└── README.md
```
