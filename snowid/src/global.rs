use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::config::MAX_MACHINE_ID;
use crate::generator::SnowIdGenerator;

static GENERATORS: Lazy<Vec<Mutex<SnowIdGenerator>>> = Lazy::new(|| {
    (0..=MAX_MACHINE_ID)
        .map(|id| Mutex::new(SnowIdGenerator::new(id)))
        .collect()
});

pub fn generate_id() -> u64 {
    generate_id_for_machine(1)
}

pub fn generate_id_for_machine(machine_id: u16) -> u64 {
    if machine_id > MAX_MACHINE_ID {
        panic!(
            "machine_id {} exceeds allowed limit {}",
            machine_id, MAX_MACHINE_ID
        );
    }
    let mut generator = GENERATORS[machine_id as usize].lock().unwrap();
    generator.generate()
}
