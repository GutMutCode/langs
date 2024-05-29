using JuMP
using HiGHS
IC = Model(HiGHS.Optimizer)

@variable(IC, xA>=0)
@variable(IC, xB>=0)
@objective(IC, Max, 4*xA + 6*xB)
@constraint(IC, 2*xA <= 14)
@constraint(IC, 3*xB <= 15)
@constraint(IC, 4*xA + 3*xB <= 36)

print(IC)

# find optimal solution
optimize!(IC)

println("Termination status: $(termination_status(IC))")
if termination_status(IC) == MOI.OPTIMAL
  println("Optimal objective value: $(objective_value(IC))")
  println("xA: ", value(xA))
  println("xB: ", value(xB))
else
  println("No optimal solution available")
end
