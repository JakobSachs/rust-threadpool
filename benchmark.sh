#!/bin/bash

# Build release version
cargo build --release

OUTPUT="benchmark_results.dat"
echo "# pool_size runtime_secs throughput_k_per_sec" > "$OUTPUT"

for size in 1 2 4 8 16 32; do
    echo "Running with pool size $size..."
    result=$(POOL_SIZE=$size ./target/release/rust-threadpool 2>&1 | tail -1)

    # Extract runtime and throughput from output like:
    # [T=4] Processed 100000000 numbers in 1.234s (12345.67 k-numbers/sec)
    runtime=$(echo "$result" | grep -oP 'in \K[0-9.]+(?=s)')
    throughput=$(echo "$result" | grep -oP '\(\K[0-9.]+(?= k-numbers)')

    echo "$size $runtime $throughput" >> "$OUTPUT"
done

echo "Results written to $OUTPUT"
