using Markdown
using InteractiveUtils

people = 10

slices = 8

avg = slices * pizzas / people = 2.5

# 3.125
pizzas = people * avg / slices

# 4
pizzas = ceil(people * avg / slices)

md"# Writing your own functions"

r = 6
A = pi * r^2

# generic function with 1 method
area(d) = pi * (d / 2)^2

# we can get the area by the diameter
A2 = area(2*r)

md"# Finding the best pizza deal"

md"## 1. How many small pizzas is the same as on XL pizza?"

dSmall = 9
dMedium = 13
dLarge = 15
dXL = 17

smalls_in_xl = area(dXL) / area(dSmall)

md"## 2. Calculate the cost per area of each pizza:"

# pizza price
pSmall = 13.10
pMedium = 20.95
pLarge = 24.90
pXL = 30.95

# cost per area
cSmall = pSmall / area(dSmall)
cMedium = pMedium / area(dMedium)
cLarge = pLarge / area(dLarge)
cXL = pXL / area(dXL)

# calculate best price
bestPrice = min(cSmall, cMedium, cLarge, cXL)

md"## 3. Is this a good deal?"

two_medium_cost = pMedium * 2 - 5
two_medium_area = area(dMedium) * 2
two_medium_deal = two_medium_cost / two_medium_area

better_deal = two_medium_deal < cXL

md"## 4. Advanced Problem"

cuts0 = 1
cuts1 = cuts0 + 1 

# maximum number of pieces the worker could make with two cuts of the pizza.
cuts2 = cuts1 + 2 = cuts0 + 1 + 2

# with three cuts
cuts3 = cuts2 + 3 = cuts0 + 1 + 2 + 3

# with four cuts
cuts4 = cuts3 + 4 = cuts0 + 1 + 2 + 3 + 4

# with `n` cuts
# cutsN = cuts0 + 1 + 2 + ... + n

a₀ = 1
a₁ = 1
a₂ = 2
a₃ = 3
a₄ = 4
a₅ = 5
a₆ = 6
a₇ = 7
a₈ = 8


cuts0 = a₀
cuts1 = a₁ + a₀
cuts2 = a₂ + a₁ + a₀
cuts3 = a₃ + (a₂ + a₁) + a₀
cuts4 = a₄ + (a₃ + a₁) + a₂ + a₀
# 5 * 3 + 1
cuts5 = a₅ + (a₄ + a₁) + (a₃ + a₂) + a₀
# 6 * 3 + 3 + 1
cuts6 = a₆ + (a₅ + a₁) + (a₄ + a₂) + a₃ + a₀
# 7 * 3 + 1
cuts7 = a₇ + (a₆ + a₁) + (a₅ + a₂) + (a₄ + a₃) + a₀
# 8 * 3 + 4 + 1
cuts8 = a₈ + (a₇ + a₁) + (a₆ + a₂) + (a₅ + a₃) + a₄ + a₀
# cutsN = custN-1 + N
