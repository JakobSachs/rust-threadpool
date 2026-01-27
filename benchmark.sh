#!/bin/bash

# Check for --force flag to re-run all benchmarks
FORCE_RERUN=false
if [[ "$1" == "--force" ]]; then
    FORCE_RERUN=true
    echo "Force re-run enabled: all benchmarks will be executed"
    echo
fi

# Build release version
cargo build --release

# Build C++ reference implementation
echo "Building C++ reference implementation..."
(cd cpp-version && make)

RESULTS_DIR="benchmark_results"
mkdir -p "$RESULTS_DIR"

# Auto-discover all bench-* binaries
BINARIES=($(ls ./target/release/bench-* 2>/dev/null | grep -v '\.d$'))

# Add C++ reference benchmark
if [ -f "cpp-version/bench" ]; then
    BINARIES+=("cpp-version/bench")
fi

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
    # Rename C++ benchmark to baseline for clarity in plots
    if [ "$name" = "bench" ]; then
        name="bench-baseline-cpp"
    fi
    output="$RESULTS_DIR/${name}.dat"

    # Skip if results already exist (unless force flag is set)
    if [ -f "$output" ] && [ "$FORCE_RERUN" = false ]; then
        echo "=== Skipping $name (results exist) ==="
        echo
        continue
    fi

    echo "=== Benchmarking $name ==="
    echo "# pool_size runtime_secs throughput_k_per_sec" > "$output"

    for size in 1 2 4 8 16; do
        echo "  Pool size $size..."

        # Run 3 times and average the results
        sum_runtime=0
        sum_throughput=0
        for run in 1 2 3; do
            if command -v numactl &> /dev/null; then
              result=$(POOL_SIZE=$size numactl --cpubind=0 --membind=0 "$bin" 2>&1 | tail -1)
            else
              echo "no numactl"
              result=$(POOL_SIZE=$size "$bin" 2>&1 | tail -1)
            fi

            # Extract runtime and throughput from output like:
            # [V1-spinlock T=4] Processed 500000000 numbers in 1.234s (12345.67 k-numbers/sec)
            runtime=$(echo "$result" | sed -n 's/.*in \([0-9.]*\)s.*/\1/p')
            throughput=$(echo "$result" | sed -n 's/.*(\([0-9.]*\) k-numbers.*/\1/p')

            sum_runtime=$(echo "$sum_runtime + $runtime" | bc)
            sum_throughput=$(echo "$sum_throughput + $throughput" | bc)
        done

        # Calculate averages (sed adds leading zero for decimals < 1)
        avg_runtime=$(echo "scale=3; $sum_runtime / 3" | bc | sed 's/^\./0./')
        avg_throughput=$(echo "scale=2; $sum_throughput / 3" | bc | sed 's/^\./0./')

        echo "$size $avg_runtime $avg_throughput" >> "$output"
    done
    echo
done

echo "Results written to $RESULTS_DIR/"
echo "Run 'gnuplot plot_benchmark.gp' to generate plot"
