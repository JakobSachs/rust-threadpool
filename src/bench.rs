pub const TASK_COUNT: u64 = 10_000_000;
pub const PRINT_INTERVAL: u64 = 333333;

pub fn collatz(mut n: u64) {
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
    // don't print everything, we want to emulate a compute-heavy task
    if original % PRINT_INTERVAL == 0 {
        print!("{original} took {steps} steps to converge\n");
    }
}
