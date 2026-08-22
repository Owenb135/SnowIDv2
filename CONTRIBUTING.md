# Contributing to SnowIDv2

First off, thank you for considering contributing to SnowIDv2! It's people like you that make open source such a great community.

## Code of Conduct

This project and everyone participating in it is governed by the [SnowIDv2 Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

1. **Fork the repository** on GitHub.
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/your-username/SnowID-V2.git
   cd SnowID-V2
   ```
3. **Set up the Rust environment**: Make sure you have `rustup` installed.
   ```bash
   rustup default stable
   ```

## Development Workflow

### Rust Core (`snowidv2`)
The core Snowflake logic is entirely in Rust.
To test changes made in `snowidv2`:
```bash
cd snowidv2
cargo test
cargo fmt
cargo clippy -- -D warnings
```

### PostgreSQL Extension (`snowidv2_pg`)
If you are changing the PostgreSQL extension, you will need the `pgrx` framework.
1. Install `cargo-pgrx`:
   ```bash
   cargo install --locked cargo-pgrx
   cargo pgrx init
   ```
2. Run tests against PostgreSQL:
   ```bash
   cd snowidv2_pg
   cargo pgrx test
   ```

### Docker Tests
You can also verify changes using the provided Docker setup:
```bash
docker compose up -d --build
docker exec -it snowidv2_postgres psql -U postgres -d snowidv2_demo
```

## Submitting a Pull Request

1. **Create a new branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/my-awesome-feature
   ```
2. **Commit your changes**. Ensure your commit messages are descriptive.
3. **Run tests, formatting, and linting**. We require all code to pass `cargo fmt` and `cargo clippy`.
4. **Push your branch** to your fork.
5. **Open a Pull Request** against the `main` branch. Provide a clear description of the problem you are solving and how your changes address it.

## Issues and Feature Requests

If you find a bug or have a feature request, please open an issue in the GitHub repository. Provide as much detail as possible, including steps to reproduce bugs and expected behavior for features.
