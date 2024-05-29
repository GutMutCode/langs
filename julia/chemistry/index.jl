# using Pkg;
# Pkg.add("Catalyst")
# Pkg.add("DifferentialEquations")
# Pkg.add("Plots")

using Catalyst
using DifferentialEquations
using Plots

# rn = @reaction_network begin
#   α, S + I --> 2I
#   β, I --> R
# end

# A simple birth-death model (parameters: b, d)
rn = @reaction_network begin
  b, 0 --> X
  d, X --> 0
end

# simluate the model

# 1. The initial condition
u0 = [:X => 1.0] # A vector which collects several different values.

# 2. The time span (start, end)
tspan = (0.0, 10.0) # A tuple

# 3. The parameters
p = [:b => 1.0, :d => 0.2]

# 4. The bundle (model, initial condition, time span, parameters)
prob = ODEProblem(rn, u0, tspan, p)

# 5. Solve
sol = solve(prob)

# 6. Plot
plot(sol)
