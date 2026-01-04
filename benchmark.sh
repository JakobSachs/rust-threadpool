#!/bin/bash

# Build release version
cargo build --release

RESULTS_DIR="benchmark_results"
mkdir -p "$RESULTS_DIR"

# Auto-discover all bench-* binaries
BINARIES=($(ls ./target/release/bench-* 2>/dev/null | grep -v '\.d$'))

if [ ${#BINARIES[@]} -eq 0 ]; then
    echo "No bench-* binaries found!"
    exit 1
fi

echo "Found ${#BINARIES[@]} pool implementations:"
for bin in "${BINARIES[@]}"; do
    echo "  - $(basename "$bin")"
done
echo

# Run benchmarks for each implementation
for bin in "${BINARIES[@]}"; do
    name=$(basename "$bin")
    output="$RESULTS_DIR/${name}.dat"

    echo "=== Benchmarking $name ==="
    echo "# pool_size runtime_secs throughput_k_per_sec" > "$output"

    for size in 1 2 4 8 16 32; do
        echo "  Pool size $size..."
        result=$(POOL_SIZE=$size "$bin" 2>&1 | tail -1)

        # Extract runtime and throughput from output like:
        # [V1-spinlock T=4] Processed 500000000 numbers in 1.234s (12345.67 k-numbers/sec)
        runtime=$(echo "$result" | grep -oP 'in \K[0-9.]+(?=s)')
        throughput=$(echo "$result" | grep -oP '\(\K[0-9.]+(?= k-numbers)')

        echo "$size $runtime $throughput" >> "$output"
    done
    echo
done

echo "Results written to $RESULTS_DIR/"
echo "Run 'gnuplot plot_benchmark.gp' to generate plot"
