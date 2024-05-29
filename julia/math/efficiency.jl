# Julia is faster than Matlab because it is able to efficiently translate its code into the same machine code as from the equivalent compilation of C code (arguably the gold-standard for efficiency)

# add macros for estimating the average time that a function takes than `@time` or `@elapesd`
# using Pkg; Pkg.add("LinearAlgebra"); Pkg.add("BenchmarkTools")
using LinearAlgebra, BenchmarkTools

# Hermitian, a useful property of a matrix to be able to exploit,
# since quicker algorithms exist for many tasks for Hermitian matrices compared to general matrices.
println(@belapsed eigvals(rand(ComplexF64,1000,1000)))
println(@belapsed (eigvals ∘ Hermitian ∘rand(ComplexF64,1000,1000)))

# Julia can also recognise properties in matrices that it isn't told. 
println(@belapsed eigvals(H) setup = (H = (Matrix ∘ Hermitian ∘ rand)(ComplexF64,1000,1000)))
