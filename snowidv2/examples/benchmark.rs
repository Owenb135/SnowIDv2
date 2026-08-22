use snowidv2::generate_id;
use snowidv2::generator::SnowIdGenerator;
use std::time::Instant;

fn main() {
    println!("===============================================================");
    println!("                  SnowIDv2 Performance Benchmark              ");
    println!("===============================================================\n");

    let iterations = 1_000_000;

    // 1. Benchmark Single-Threaded Raw SnowIdGenerator
    {
        let mut generator = SnowIdGenerator::new(1);
        let start = Instant::now();
        for _ in 0..iterations {
            let _id = generator.generate();
        }
        let elapsed = start.elapsed();
        let ids_per_sec = (iterations as f64) / elapsed.as_secs_f64();
        let nanos_per_id = elapsed.as_nanos() as f64 / (iterations as f64);

        println!("1. Single-Threaded Generator (`SnowIdGenerator::generate`):");
        println!("   - Iterations:      {:>12}", iterations);
        println!("   - Elapsed Time:    {:>12.2?}", elapsed);
        println!("   - Throughput:      {:>12.0} IDs/sec", ids_per_sec);
        println!("   - Latency per ID:  {:>12.2} ns/ID\n", nanos_per_id);
    }

    // 2. Benchmark Global Thread-Safe Generator (`generate_id()`)
    {
        let start = Instant::now();
        for _ in 0..iterations {
            let _id = generate_id();
        }
        let elapsed = start.elapsed();
        let ids_per_sec = (iterations as f64) / elapsed.as_secs_f64();
        let nanos_per_id = elapsed.as_nanos() as f64 / (iterations as f64);

        println!("2. Thread-Safe Global Generator (`snowid::generate_id()`):");
        println!("   - Iterations:      {:>12}", iterations);
        println!("   - Elapsed Time:    {:>12.2?}", elapsed);
        println!("   - Throughput:      {:>12.0} IDs/sec", ids_per_sec);
        println!("   - Latency per ID:  {:>12.2} ns/ID\n", nanos_per_id);
    }

    // 3. Multi-Threaded Concurrent Generation Benchmark (8 Threads)
    {
        use std::thread;
        let num_threads = 8;
        let iterations_per_thread = iterations / num_threads;
        let start = Instant::now();

        let mut handles = vec![];
        for thread_idx in 0..num_threads {
            let handle = thread::spawn(move || {
                for _ in 0..iterations_per_thread {
                    let _id = snowidv2::global::generate_id_for_machine((thread_idx % 64) as u16);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let elapsed = start.elapsed();
        let ids_per_sec = (iterations as f64) / elapsed.as_secs_f64();
        let nanos_per_id = elapsed.as_nanos() as f64 / (iterations as f64);

        println!(
            "3. Multi-Threaded Concurrent Generation ({} Threads across machines):",
            num_threads
        );
        println!("   - Iterations:      {:>12}", iterations);
        println!("   - Elapsed Time:    {:>12.2?}", elapsed);
        println!("   - Throughput:      {:>12.0} IDs/sec", ids_per_sec);
        println!("   - Latency per ID:  {:>12.2} ns/ID\n", nanos_per_id);
    }

    println!("===============================================================");
}
