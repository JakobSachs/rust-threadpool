# ============================================================
# CONFIGURATION - Adjust these to filter implementations
# ============================================================

# Output file name
OUTPUT_FILE = "benchmark_plot.svg"

# Plot title
TITLE = "Threadpool Performance Comparison"

# Index range for implementations (1-based, excludes baseline which is always shown)
# Set both to 0 to include all implementations
START_INDEX = 5
END_INDEX = 6

# ============================================================

set terminal svg size 1200,700 enhanced font 'monospace,14' background '#ffffff'
set output OUTPUT_FILE

set title TITLE font ',24'
set xlabel 'Pool Size (threads)' font ',18'
set ylabel 'Runtime (seconds)' font ',18'
set key font ',18'

set logscale xy
set xrange [1:32]
set yrange [0.7:8]

set xtics (1, 2, 4, 8, 16, 32)
set grid
set key outside right top

# ============================================================
# STYLE - Cohesive color palette (Tableau 10 inspired)
# ============================================================

# Baseline style: solid dark gray, thick line, filled circles
BASELINE_COLOR = '#555555'
BASELINE_LW = 3
BASELINE_PT = 5   # filled circle
BASELINE_PS = 1.5

# Implementation palette: consistent style, varying colors
IMPL_LW = 2
IMPL_PT = 7       # filled circle for all
IMPL_PS = 1.2

set style line 1 lc rgb '#4E79A7' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # steel blue
set style line 2 lc rgb '#F28E2B' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # orange
set style line 3 lc rgb '#E15759' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # coral red
set style line 4 lc rgb '#76B7B2' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # teal
set style line 5 lc rgb '#59A14F' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # green
set style line 6 lc rgb '#EDC948' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # gold
set style line 7 lc rgb '#B07AA1' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # purple
set style line 8 lc rgb '#FF9DA7' lw IMPL_LW pt IMPL_PT ps IMPL_PS  # pink

# Baseline style
set style line 100 lc rgb BASELINE_COLOR lw BASELINE_LW pt BASELINE_PT ps BASELINE_PS

# ============================================================

# Baseline file (always included)
BASELINE = "benchmark_results/bench-baseline-cpp.dat"

# Get non-baseline files, sorted
ALL_FILES = system("ls benchmark_results/bench-v*.dat 2>/dev/null | sort -V")

# Filter files based on index range (awk is 1-based)
if (START_INDEX > 0 && END_INDEX > 0) {
    FILES = system(sprintf("ls benchmark_results/bench-v*.dat 2>/dev/null | sort -V | awk 'NR>=%d && NR<=%d'", START_INDEX, END_INDEX))
} else {
    FILES = ALL_FILES
}

# Plot baseline first (solid, prominent), then filtered implementations
plot BASELINE using 1:2 with linespoints ls 100 title "baseline (C++)", \
     for [i=1:words(FILES)] word(FILES,i) using 1:2 with linespoints ls i \
         title system(sprintf("basename %s .dat | sed 's/bench-//'", word(FILES,i)))
