pub mod config;
pub mod decode;
pub mod generator;
pub mod global;
pub mod time;

#[cfg(test)]
mod test;

pub use decode::decode;
pub use global::{generate_id, generate_id_for_machine};

/// The pure SQL implementation of the Snowflake generator for PostgreSQL.
/// This allows you to apply the SQL functions directly to your database from your Rust backend
/// (using SQLx, Diesel, etc.) without needing to download any separate .sql files.
pub const POSTGRES_PURE_SQL: &str = include_str!("../../sql/postgres_pure.sql");
