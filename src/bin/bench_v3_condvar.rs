use rust_threadpool::bench::{TASK_COUNT, collatz};
use rust_threadpool::pools::v3_condvar;

fn main() {
    use std::time::Instant;

    // get pool size from env
    let pool_size = std::env::var("POOL_SIZE")
        .unwrap_or("1".to_string())
        .parse()
        .unwrap();
    let pool = v3_condvar::Pool::new(pool_size);

    let start = Instant::now();
    for i in 0..TASK_COUNT {
        pool.submit(move || {
            collatz(i);
        });
    }

    pool.join_all();
    let elapsed = start.elapsed();

    let per_second = TASK_COUNT as f64 / (elapsed.as_secs_f64() * 1000.0);

    println!(
        "[V3-condvar T={}] Processed {} numbers in {:.3}s ({:.2} k-numbers/sec)",
        pool_size,
        TASK_COUNT,
        elapsed.as_secs_f64(),
        per_second
    );
}
