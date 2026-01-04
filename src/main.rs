pub mod pool;

fn collatz(mut n: u64) {
    if n == 0 {
        return;
    }
    let original = n;
    let mut steps: u32 = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    // don't print everything, we want to emulate a compuyte-heavy task
    if original % 333333 == 0 {
        print!("{original} took {steps} steps to converge\n");
    }
}

fn main() {
    use std::time::Instant;

    // get pool size from env
    let pool_size = std::env::var("POOL_SIZE")
        .unwrap_or("1".to_string())
        .parse()
        .unwrap();
    let pool = pool::Pool::new(pool_size);
    let count = 500_000_000u64;

    let start = Instant::now();
    pool.submit_iter(
        (move |i| {
            collatz(i);
        })
        .into(),
        0..count,
    );

    pool.join_all();
    let elapsed = start.elapsed();

    let per_second = count as f64 / (elapsed.as_secs_f64() * 1000.0);

    println!(
        "[T={}] Processed {} numbers in {:.3}s ({:.2} k-numbers/sec)",
        pool_size,
        count,
        elapsed.as_secs_f64(),
        per_second
    );
}
