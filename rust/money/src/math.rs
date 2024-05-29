#![allow(unused)]
enum Number {}
enum Zero {}
/// Used to describe quantities greather than zero
enum Positive {}
/// Used to describe quantities less than zero
enum Nagative {}
enum Sign {
    Positive,
    Nagative,
}

enum Forever {}
enum Limited {}

enum NumberLine {}

enum NaturalNumber {
    Positive,
}

struct WholeNumber {
    zero: Zero,
    natural_number: NaturalNumber,
}

struct Integer {
    whole_number: WholeNumber,
    nagative: Nagative,
}

struct RationalNumber {
    kind: f64,
    integer: Integer,
    decimal: Limited,
}

struct IrrationalNumber {
    kind: f64,
    decimal: Forever,
}

struct RealNumber {
    rational: RationalNumber,
    irrational: IrrationalNumber,
}
