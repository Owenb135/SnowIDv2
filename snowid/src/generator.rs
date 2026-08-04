use crate::config::*;
use crate::time::current_time_millis;

pub struct SnowIdGenerator {
    machine_id: u16,
    sequence: u16,
    last_timestamp: u64,
}

impl SnowIdGenerator {
    pub fn new(machine_id: u16) -> Self {
        if machine_id > MAX_MACHINE_ID {
            panic!("machine_id exceeds allowed limit")
        }

        Self {
            machine_id,
            sequence: 0,
            last_timestamp: 0,
        }
    }

    pub fn generate(&mut self) -> u64 {
        let mut timestamp = current_time_millis();

        if timestamp < self.last_timestamp {
            timestamp = self.last_timestamp;
        }

        //same millis -> increase sequence
        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & MAX_SEQUENCE;

            // sequence overflow -> wait for new ms
            if self.sequence == 0 {
                while timestamp <= self.last_timestamp {
                    timestamp = current_time_millis();
                }
            }
        } else {
            //new millis -> reset sequence
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        // BUild final id
        ((timestamp - EPOCH) << (MACHINE_ID_BITS + SEQUENCE_BITS))
            | ((self.machine_id as u64) << SEQUENCE_BITS)
            | (self.sequence as u64)
    }
}
