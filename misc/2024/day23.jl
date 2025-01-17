#!/usr/bin/env julia
# using Pkg
# Pkg.add("DrawSimpleGraphs")
# Pkg.add("Plots")
# Pkg.add("SimpleGraphs")
using DrawSimpleGraphs, JSON, Plots, SimpleGraphs

# Load the JSON file
json_path = "day23.json"
json_data = JSON.parsefile(json_path)

# Initialize a graph
g = UndirectedGraph()

# Add edges from JSON data
for edge in json_data
    add!(g, edge[1], edge[2])
end

# Plot the graph
embed(g, :spring)
set_vertex_size(g, 1)
draw(
    g,
)
savefig("day23.png")
run(`pkill -9 gksqt`)
