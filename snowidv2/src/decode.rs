use crate::config::*;

pub fn decode(id: u64) -> (u64, u16, u16) {
    let timestamp = (id >> (MACHINE_ID_BITS + SEQUENCE_BITS)) + EPOCH;

    let machine_id = ((id >> SEQUENCE_BITS) & ((1 << MACHINE_ID_BITS) - 1)) as u16;

    let sequence = (id & ((1 << SEQUENCE_BITS) - 1)) as u16;

    (timestamp, machine_id, sequence)
}
