#!/bin/bash
# Adds dark mode CSS to an SVG file

input_file="$1"
output_file="$2"

# Read the entire file and insert CSS after the first occurrence of xmlns...>
awk '
BEGIN { inserted = 0 }
{
    print
    if (!inserted && />/ && /xmlns/) {
        print "<style>"
        print "  /* Light mode (default) */"
        print "  @media (prefers-color-scheme: light) {"
        print "    .gnuplot_canvas { fill: #ffffff !important; }"
        print "    text { fill: #000000 !important; }"
        print "    .gnuplot_plot_1 path, .gnuplot_plot_1 circle { stroke: #555555 !important; fill: #555555 !important; }"
        print "    line, polyline, polygon, path, rect { stroke: #000000 !important; }"
        print "  }"
        print "  /* Dark mode */"
        print "  @media (prefers-color-scheme: dark) {"
        print "    svg { background: #1e1e1e !important; }"
        print "    .gnuplot_canvas { fill: #1e1e1e !important; }"
        print "    rect.background { fill: #1e1e1e !important; }"
        print "    text { fill: #e0e0e0 !important; }"
        print "    .gnuplot_plot_1 path, .gnuplot_plot_1 circle { stroke: #888888 !important; fill: #888888 !important; }"
        print "    .gnuplot_plot_2 path, .gnuplot_plot_2 circle { stroke: #6EAAD7 !important; fill: #6EAAD7 !important; }"
        print "    .gnuplot_plot_3 path, .gnuplot_plot_3 circle { stroke: #FFB570 !important; fill: #FFB570 !important; }"
        print "    .gnuplot_plot_4 path, .gnuplot_plot_4 circle { stroke: #FF8A8C !important; fill: #FF8A8C !important; }"
        print "    .gnuplot_plot_5 path, .gnuplot_plot_5 circle { stroke: #9DD4D0 !important; fill: #9DD4D0 !important; }"
        print "    .gnuplot_plot_6 path, .gnuplot_plot_6 circle { stroke: #7BC77C !important; fill: #7BC77C !important; }"
        print "    .gnuplot_plot_7 path, .gnuplot_plot_7 circle { stroke: #FFE083 !important; fill: #FFE083 !important; }"
        print "    line.gridline, polyline.gridline { stroke: #444444 !important; }"
        print "    line, polyline { stroke: #666666 !important; }"
        print "  }"
        print "</style>"
        inserted = 1
    }
}
' "$input_file" > "$output_file"
