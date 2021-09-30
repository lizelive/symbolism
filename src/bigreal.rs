use std::ops::Neg;

use num::{BigInt, BigRational, Float, Num, NumCast};


struct Precision<T> where T: Float{
    value: T,
    precision: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BigReal{
    base: usize,
    exponent: isize,
    significand: isize,
    // how many signifigat figures we have
    sig_figures: isize,
}

