use std::{cmp::{max, min}, ops::{Neg, Shl, Sub}};

use num::{BigInt, BigRational, Float, Num, NumCast};

struct Precision<T>
where
    T: Float,
{
    value: T,
    precision: f64,
}

type TExp = isize;
type TSig = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BigReal {
    exponent: TExp,
    significand: TSig,

    positive: bool,

    // how many signifigat figures we have
    specfied_bits: usize,
    max_specfied: usize,
    signficant_bits: usize,
}


fn trunc(x: isize, bits: usize) -> isize {
    let mask = (1 as isize).shl(bits).sub(1);
    x & mask
}

impl BigReal {
    fn signum(&self) -> i8 {
        let is_zero = self.exponent == 0;
        if is_zero {
            0
        } else {
            match self.positive {
                true => 1,
                false => -1,
            }
        }
    }

    fn pow(&self, other: &Self) -> Self {
        todo!()
    }

    fn precision(&self) -> f64 {
        (self.specfied_bits as f64).log10()
    }

    #[inline]
    fn with_exponent(&self, exponent: TExp) -> Self {
        if exponent == self.exponent {
            *self
        } else {
            let shift_by = self.exponent - exponent;
            let specfied_bits = max(self.specfied_bits as isize + shift_by, 0) as usize;
            let significand = self.significand.shl(shift_by);
            let signficant_bits = min(specfied_bits, self.signficant_bits);
            BigReal {
                exponent,
                max_specfied: self.max_specfied,
                positive: self.positive,
                significand,
                specfied_bits,
                signficant_bits,
            }
        }
    }
    fn add(&self, other: &Self) {
        let new_exponent = max(self.exponent, other.exponent) + 1;
        let a = self.with_exponent(new_exponent);
        let b = self.with_exponent(new_exponent);

        let specfied_bits = min(self.specfied_bits, other.specfied_bits);
    }
}
