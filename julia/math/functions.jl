function f(x,y)
  return √(x^2 + y^2)
end

println(f(3,4))

# alternative syntax
g(x,y) = √(x^2 + y^2)

println(g(3,4))

# The expression after the equals must be a single line,
# but can also be extended to longer expressions
# by a `begin`-`end` block
h(x) = begin
  # multiple comparison in the same statement
  if 0 < x < 3
    return x
  else
    return 0
  end
end

println(h(2))
println(h(-2))

# `circle` for function composition hg(x, y) = h(g(x, y))
hg = h ∘ g
println(hg(1,1))
println(hg(3,5))

# `2d` is numeric coefficient to denote `2 * d`
d = 2; e = 2d

# This only works with numeric literals, i.e. actual numbers,
# not variables which take a number as a value.
# `de` isn't `d * e`
de = 2;

# Short-circuiting
# you can use logical operators as if-then-else
3 > 4 && "This shouldn't be printed"
3 < 4 || "This shouldn't be printed"

# ternary operator
3 > 4 ? "True" : "False"

# broadcasting
p(x) = x^3 - 4x^2 + 2x

A = [ [1,2,3] [4,5,6] [7,8,9] ]

# Defined by squaring and cubing `A`
p(A)

# Apply `p` to each element
broadcast(p, A)

p.(A)

# switch row and column
+(A)

# add all elements
+(A...)

# arbitrary number of arguments
function myminimum(x...)
  m = Inf
  for y ∈ x
    if y < m
      m = y
    end
  end
  return m
end

myminimum(12,15,9,151,23)
