#[cfg(test)]
mod tests {
  use crate::generator::SnowIdGenerator;
  use crate::decode::decode;

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
}