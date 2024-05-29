# Macros to store expressions in variables
# preprocessing
q = quote
  if 3 < 4
    return true
  else
    return false
  end
end;

# `Expr` type
typeof(q)

# evaluate stored expression
eval(q)

# create `Symbol` generating function
function symbolFn(n)
  # convert `n` to subscript
  subscriptn = String([Char(0x2080+d) for d in reverse(digits(n))])
  # concatenate `F` and `subscriptn` into Symbol (IDentifier)
  return Symbol("F",subscriptn)
end

# create Symbol: F₁₂₃
symbolFn(123)

# create `Fₙ` variables by using loop
F₁ = 1
F₂ = 1
for n ∈ 3:50
  eval(:(
          $(symbolFn(n)) = $(symbolFn(n-1)) + $(symbolFn(n-2)) 
      ))
end

# create symbols through broadcasting `.` and print its name and value
# use [] for looping
# [println( String(s), " = $(eval(:($s)))") for s ∈ symbolFn.(1:15)]

# println(F₅₀)

# Macros are distingushed as such by an `@` preceding their name.
macro multiple(n::Integer, expr)
  return quote
    # enumerate all by `...`
    $([esc(expr) for i ∈ 1:n]...)
  end
end

x = 1
@multiple 10 x += 1
# println(x)


# You can get the expressions by @macroexpand before is evaluated 
multiple = @macroexpand @multiple 10 x += 1
# println(multiple)

# @elapsed calculate times how long its argument takes to run in seconds.
# inv([[1,0] [0,1]])
@elapsed inv(rand(1000,1000))

# @time print all times how long its arguments takes to run in seconds
@time 2 * 2

# Whan the first time a functions runs, Julia has to compile it first, so it will always be slower than subsequent runs.
# So use `BenchmarkTools` for testing

