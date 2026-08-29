use pgrx::prelude::*;

// import core functions
use snowidv2_core::{decode, generate_id, generate_id_for_machine};

pgrx::pg_module_magic!();

/// Generate a Snowflake ID using default machine ID (1).
/// Can be used as a table column default: `id BIGINT PRIMARY KEY DEFAULT snowidv2()`
#[pg_extern]
fn snowidv2() -> i64 {
    generate_id() as i64
}

/// Generate a Snowflake ID for a specific machine ID (0..63).
/// Can be used as a table column default: `id BIGINT PRIMARY KEY DEFAULT snowidv2_with_machine(2)`
#[pg_extern]
fn snowidv2_with_machine(machine_id: i32) -> i64 {
    if machine_id < 0 || machine_id > 63 {
        error!("Invalid machine ID {}: must be between 0 and 63", 
             machine_id);
    }
    generate_id_for_machine(machine_id as u16) as i64
}

/// Decode a Snowflake ID into its timestamp (ms since UNIX epoch), machine_id, and sequence.
/// Example: `SELECT * FROM snowidv2_decode(123456789012345678);`
#[pg_extern]
fn snowidv2_decode(
    id: i64,
) -> TableIterator<
    'static,
    (
        name!(timestamp_ms, i64),
        name!(machine_id, i32),
        name!(sequence, i32),
    ),
> {
    let (ts, mid, seq) = decode(id as u64);
    TableIterator::once((ts as i64, mid as i32, seq as i32))
}
