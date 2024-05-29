# Array

## Align

Julia arrays are not copied when assigned to another variable.  
 After `A = B`, changing elements of `B` will modify `A` as well.
Updating operators like `+=` do not operate in-place, they are equivalent to `A = A + B` which rebinds the _left-hand side_ to the result of the _right-hand side_ expression.

## Indexing

- Julia uses 1-based indexing.
- Julia arrays are indexed with square brackets, and can have more than one dimension `A[i, j]`.  
  This syntax is not just syntactic sugar for a reference to a pointer or address as in C/C++.

## Order

Julia arrays are column major (Fortran ordered)
whereas C/C++ arrays are row major ordered by default.

# Variables

# Function Arguments

Julia values are not copied when assigned or passed to a function.
If a function modifies an array, the changes will be visible in the caller.

# Whitespace

Whitespace is significant in Julia.

# Types

## Automatic Promoting

Literal number without a decimal point create signed integers, of type `Int`,
but literals too large to fit in the machine word size will automatically be promoted to larger size type, such as `Int64` (if `Int` is `Int32`), `Int128`, or the arbitrarily large `BigInt` type.

## Suffixes

There are no numeric literal suffixes, such as `L`, `LL`, `U`, `UL`, `ULL` to indicate unsigned and/or signed vs. unsigned.
