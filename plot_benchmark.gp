set terminal pngcairo size 800,500 enhanced font 'Arial,12'
set output 'benchmark_plot.png'

set title 'Threadpool Performance vs Pool Size'
set xlabel 'Pool Size (threads)'
set ylabel 'Runtime (seconds)'

set xtics (1, 2, 4, 8, 16, 32)
set grid

plot 'benchmark_results.dat' using 1:2 with linespoints \
        lw 2 pt 7 ps 1.5 lc rgb '#0072B2' notitle
