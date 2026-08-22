// custom epoch (same as nodejs)

pub const EPOCH: u64 = 1700000000000;

//bit allocation
pub const MACHINE_ID_BITS: u8 = 6;
pub const SEQUENCE_BITS: u8 = 14;

//derived values
pub const MAX_SEQUENCE: u16 = (1 << SEQUENCE_BITS) - 1;
pub const MAX_MACHINE_ID: u16 = (1 << MACHINE_ID_BITS) - 1;
