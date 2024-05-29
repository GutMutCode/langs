# https://en.wikipedia.org/wiki/Logistic_map
# xₙ₊₁ = rxₙ(1 - xₙ)
# where x₀ ∈ [0,1] and r ∈ [0,4]
using Pkg;
Pkg.add("Plots");
using Plots.plot

logisticiteration(r,x) = r * x * (1-x)

N = 100
X₀ = 0.5
r = 3.1

sequence = zero(N + 1); sequence[1] = X₀;
for n ∈ 1:N
  sequence[n+1] = logisticiteration(r, sequence[n])
end

plot(0:N, sequence, legend=false)

# counting from 0 to 4 in intervals of 0.005
rvalues = 0.0:0.005:4.0

discardediterations = 200

maxocillatoryvalues = 100

begin
  bifurcationreset
  points = Vector{Tuple{Float64, Float64}}(undef, 0);
end;

for rvalue in rvalues
  x = 0.5
  for i ∈ 1:discardediterations
    x = logisticiteration(rvalue, x)
  end
  maxdiscardedvalue = x
  for i ∈ 1:maxocillatoryvalues
    x = logisticiteration(rvalue, x)
    push!(points, (rvalue, x))
    if abs(x - maxdiscardedvalue) < 0.001
      break;
    end
  end
end

bifurcationplot = scatter(points, markersize = 1, markercolor = :black, legend = false, xlims = (min(rvalues...), max(rvalues...)), ylims = (-0.5, 1.5))
