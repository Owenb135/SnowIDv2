#[cfg(test)]

mod tests {
    use crate::config::SEQUENCE_BITS;
    use crate::config::*;
    use crate::decode::decode;
    use crate::generator::SnowIdGenerator;

    #[test]
    fn test_id_increasing() {
        let mut generate = SnowIdGenerator::new(1);

        let id1 = generate.generate();
        let id2 = generate.generate();

        assert!(id2 > id1);
    }

    #[test]
    fn test_decode() {
        let mut generate = SnowIdGenerator::new(1);
        let id = generate.generate();

        let (ts, machine_id, _seq) = decode(id);

        assert!(ts > 0);
        assert_eq!(machine_id, 1);
    }

    #[test]
    fn test_global_generator() {
        use crate::global::generate_id_for_machine;
        let id_m2 = generate_id_for_machine(2);
        let (_, machine_id, _) = decode(id_m2);
        assert_eq!(machine_id, 2);
    }

    #[test]
    fn test_sequence_overflow() {
        let mut gener = SnowIdGenerator::new(1);
        let mut previous_id: u64 = 0;
        for _id_num in 1..=20000 {
            let id_generated = gener.generate();
            assert!(id_generated > previous_id);
            previous_id = id_generated;
        }
    }

    #[test]
    fn test_known_id() {
        let known_timestamp_offset: u64 = 12345;
        let known_machine_id: u16 = 3;
        let known_sequence: u16 = 7;

        let secondary_id = (known_timestamp_offset << (MACHINE_ID_BITS + SEQUENCE_BITS))
            | ((known_machine_id as u64) << SEQUENCE_BITS)
            | known_sequence as u64;

        let (result_timestamp, result_machine_id, result_sequence) = decode(secondary_id);

        assert_eq!(result_timestamp, known_timestamp_offset + EPOCH);
        assert_eq!(result_machine_id, known_machine_id);
        assert_eq!(result_sequence, known_sequence);
    }
}
