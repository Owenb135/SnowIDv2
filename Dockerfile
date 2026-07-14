# Default PostgreSQL major version (supports 16, 17, 18)
ARG PG_MAJOR=17

# ==============================================================================
# Stage 1: Builder
# ==============================================================================
FROM postgres:${PG_MAJOR}-bookworm AS builder

# Re-declare ARG for use inside builder stage
ARG PG_MAJOR

# Install necessary build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    clang \
    llvm-dev \
    libreadline-dev \
    zlib1g-dev \
    flex \
    bison \
    postgresql-server-dev-${PG_MAJOR} \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup
ENV RUSTUP_HOME=/root/.rustup \
    CARGO_HOME=/root/.cargo \
    PATH=/root/.cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# Install cargo-pgrx matching version 0.18.0 from Cargo.toml
RUN cargo install --locked cargo-pgrx --version 0.18.0

# Initialize pgrx with the system pg_config
RUN cargo pgrx init --pg${PG_MAJOR}=/usr/lib/postgresql/${PG_MAJOR}/bin/pg_config

# Set working directory
WORKDIR /usr/src/snowid

# Copy entire workspace
COPY . .

# Build and install the pgrx extension for the target PostgreSQL version
WORKDIR /usr/src/snowid/snowid_pg
RUN cargo pgrx install --release --no-default-features --features pg${PG_MAJOR}

# ==============================================================================
# Stage 2: Runtime
# ==============================================================================
FROM postgres:${PG_MAJOR}-bookworm AS runtime

ARG PG_MAJOR

# Copy built extension library and SQL/control files from builder stage
COPY --from=builder /usr/lib/postgresql/${PG_MAJOR}/lib/*snowid* /usr/lib/postgresql/${PG_MAJOR}/lib/
COPY --from=builder /usr/share/postgresql/${PG_MAJOR}/extension/snowid* /usr/share/postgresql/${PG_MAJOR}/extension/

# Copy initialization scripts so the extension and pure SQL functions are auto-loaded on first boot
COPY docker/initdb/ /docker-entrypoint-initdb.d/
COPY sql/postgres_pure.sql /docker-entrypoint-initdb.d/02-postgres_pure.sql
COPY sql/ /usr/local/share/snowid/sql/

# Set default environment variables (can be overridden)
ENV POSTGRES_DB=snowid_demo
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres

EXPOSE 5432
