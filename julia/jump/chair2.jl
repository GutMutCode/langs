#***********************************************************************
## Incrediblex Chairs 2 assignment, Simple LP
using JuMP
using HiGHS
#***********************************************************************

#***********************************************************************
# Data
Chairs = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"]
C = length(Chairs)
ProductionLines = [1, 2, 3, 4, 5]
P = length(ProductionLines)

Profit = [6, 5, 9, 5, 6, 3, 4, 7, 4, 3] # profit for each chair
Capacity = [47, 19, 36, 13, 46] # production line capacity
RecourseUsage = [ # RecourseUsage[p, c] res use for chair c on prod. line p
6  4 2  3 1 10 2 9  2 5;
5  6 1  1 7  2 9 1  8 6;
8 10 7  2 9  6 9 6  5 6;
8  4 8 10 5  4 1 5  3 5;
1  4 7  2 4  1 2 3 10 1]
#***********************************************************************

#***********************************************************************
# Model
IC2 = Model(HiGHS.Optimizer)

# x[c] is number of chairs produced on line c
@variable(IC2, x[c=1:C] >= 0)

# Objective is maximize profit
@objective(IC2, Max, sum( Profit[c] * x[c] for c=1:C ) )

# Constraints ensuring production line capacity is not exceeded
# apply constraint for each production line [p=1:P]
@constraint(IC2, [p=1:P],
            sum( RecourseUsage[p,c] * x[c] for c=1:C) <= Capacity[p] )

#***********************************************************************
# Solve - transfer to HiGHS
optimize!(IC2)
println("Termination status: ", termination_status(IC2))
#***********************************************************************
if termination_status(IC2) == MOI.OPTIMAL
  println("Optimal objective value: \$(objective_value(IC2))")
  for c=1:C
    if value(x[c]) > 0.001 # only print the chairs actually produced
      println("No of chairs of type ", Chairs[c], " produced: ",
              value(x[c]))
    end
  end
else
  # model is in-feasible or un-bounded
  println("No optimal solution available")
end
#***********************************************************************
