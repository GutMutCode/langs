x, ___ = size([s s; 1 1])

a² = 10

a = b = 1 # a: 1, b: 1
a = (b = 2+2) + 3 # b: 4, a: 7

a = [1,2,3] # mutable an existing value
b = a
a[1] = 42 # array index is based on the number 1
a = 3.14159 # b: [42,2,3]

a.field = "value" # setproperty! for mutable structe

1 # integer literal => binary in-memory repersentations as objects: numeric primitive
1.0 # floating point literal

typeof(1) # default type is depends on the architecture ( Int32, Int64 )

Sys.WORD_SIZE # target architecture size

typeof(3000000000) # always Int64 (cannot represents to Int32)

# determining storage size based on a literal
typeof(0x01) # UInt8
typeof(0x0001) # UInt16

# signed number with `-` prefix 
-0x2 # 0xfe (using two's complement)
-0x0002 # 0xfffe

# typemin & typemax
(typemin(Int32), typemax(Int32))

# overflow behavior
x = typemax(Int64) + 1 # x: -9223372036854775808 == typemin(Int64) # cycle

10^19 # wrap around by overflow behavior

big(10)^19 # 10^19 - big: arbitrary precision number type of BigInt

f64 = 1e10 # 1.0e10

typeof(f64) # Float64

f32 = .5f0 # 0.5f0 (half-precision floating point)
typeof(f32) # Float32

x = Float32(f64) # 1.0f10

# Hexadecimal floating point
0x1.8p3 # 12.0 (with p preceding base-2 exponent)

# type promotion
2*Float16(2) # Float16
2*Float16(2.) # Float16

# separate digit by the underscore
1_000_000 # 1000000
0.000_000_005 # 5.0e-9
0xdead_beef # 0xdeadbeef
0b1011_0010 # 0b10110010

# floating-point zero
0.0 == -0.0 # true
bitstring(0.0) == bitstring(-0.0) # false (represents differently)

Inf # positive infinity of Float64 (Inf16, Inf32 is also supported)
-Inf # negative infinity of Float64
NaN # not a number of Float64

1/Inf # 0.0

1/0 # Inf
500 + Inf # Inf
Inf * Inf # Inf

-5/0 # -Inf
500 - Inf # -Inf

0/0 # NaN
Inf - Inf # NaN
Inf / Inf # NaN

NaN == NaN # false
NaN < NaN # false
NaN > NaN # false

(typemin(Float16), typemax(Float16)) # (-Inf16, Inf16) ans so on (-Inf32, Inf32)...
