use pgrx::prelude::*;

// import core functions
use snowid_core::{generate_id, generate_id_for_machine, decode};

pgrx::pg_module_magic!();

/// Generate a Snowflake ID using default machine ID (1).
/// Can be used as a table column default: `id BIGINT PRIMARY KEY DEFAULT snowid()`
#[pg_extern]
fn snowid() -> i64 {
    generate_id() as i64
}

/// Generate a Snowflake ID for a specific machine ID (0..63).
/// Can be used as a table column default: `id BIGINT PRIMARY KEY DEFAULT snowid_with_machine(2)`
#[pg_extern]
fn snowid_with_machine(machine_id: i32) -> i64 {
    if machine_id < 0 || machine_id > 63 {
        panic!("machine_id must be between 0 and 63, got {}", machine_id);
    }
    generate_id_for_machine(machine_id as u16) as i64
}

/// Decode a Snowflake ID into its timestamp (ms since UNIX epoch), machine_id, and sequence.
/// Example: `SELECT * FROM snowid_decode(123456789012345678);`
#[pg_extern]
fn snowid_decode(
    id: i64,
) -> TableIterator<'static, (name!(timestamp_ms, i64), name!(machine_id, i32), name!(sequence, i32))> {
    let (ts, mid, seq) = decode(id as u64);
    TableIterator::once((ts as i64, mid as i32, seq as i32))
}
