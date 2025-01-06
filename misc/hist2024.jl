#!/usr/bin/env julia
import Pkg
Pkg.add("JSON")
Pkg.add("Plots")
Pkg.add("KernelDensity")
using JSON, Plots, KernelDensity

data2024 = read("2024/execution_time.json", String)
data = JSON.parse(data2024)
times = [item["time"] for item in data if haskey(item, "time")]
tl = log2.(times)
ktd = kde(tl)

# histogram(tl, xlims=(-2, 20), bins=10, title="AoC2024", xlabel="時間", ylabel="Freq", legend=false, fontfamily="Noto Sans CJK JP")
histogram(tl, xlims=(-5, 20), ylims=(0, 0.4), bins=20, normalize=true, title="AoC2024 execution time distribution in Rust", xlabel="log2(time[ms])", ylabel="Freq", legend=false)
plot!(ktd.x, ktd.density, ylims=(0, 0.4), color=:red, linewidth=2)
savefig("hist2024.png")
