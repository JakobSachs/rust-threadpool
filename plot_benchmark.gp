set terminal pngcairo size 1000,600 enhanced font 'Arial,12'
set output 'benchmark_plot.png'

set title 'Threadpool Performance Comparison'
set xlabel 'Pool Size (threads)'
set ylabel 'Runtime (seconds)'

set xtics (1, 2, 4, 8, 16, 32)
set grid
set key outside right top

# Color palette for different implementations
set linetype 1 lc rgb '#0072B2' lw 2 pt 7  ps 1.5  # blue
set linetype 2 lc rgb '#D55E00' lw 2 pt 9  ps 1.5  # orange
set linetype 3 lc rgb '#009E73' lw 2 pt 5  ps 1.5  # green
set linetype 4 lc rgb '#CC79A7' lw 2 pt 11 ps 1.5  # pink
set linetype 5 lc rgb '#F0E442' lw 2 pt 13 ps 1.5  # yellow
set linetype 6 lc rgb '#56B4E9' lw 2 pt 15 ps 1.5  # light blue

# Auto-discover and plot all .dat files in benchmark_results/
FILES = system("ls benchmark_results/*.dat 2>/dev/null")

# Build plot command dynamically
plot for [file in FILES] file using 1:2 with linespoints \
    title system(sprintf("basename %s .dat | sed 's/bench-//'", file))
