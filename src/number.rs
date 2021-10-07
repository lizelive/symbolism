use std::{cmp::Ordering};

use num::{BigInt, BigRational, traits::real::Real};

use crate::context::Symbol;

type AnyNumber = Box<dyn Number>;
type AnyComplex = Box<dyn Complex>;
type AnyInteger = Box<dyn Integer>;
type AnyRational = Box<dyn Rational>;
type AnyReal = Box<dyn Real>;

pub enum DirectedInfinity {
    Known(AnyComplex),
    Unkown,
}

trait Integer: Rational {
    fn gcd(self, other: &AnyInteger) -> AnyInteger;
}

trait Rational: Real {
    fn numerator(&self) -> AnyInteger;
    fn denominator(&self) -> AnyInteger;
    fn numerator_denominator(&self) -> (AnyInteger, AnyInteger){
        (self.numerator(), self.denominator())
    }
    //fn numerator_denominator(&self, rhs: AnyNumber) -> AnyNumber;
}

trait Real: Complex {
    fn real_digits(&self, base: &AnyInteger) -> (Vec<AnyInteger>, AnyInteger);
    fn integer_part(&self) -> AnyInteger;
    fn fractional_part(&self) -> AnyReal;

    fn real_floor(&self) -> AnyInteger;
    fn real_ceil(&self) -> AnyInteger;


}

trait Complex: Number {
    //fn log(&self, base: AnyComplex) -> AnyComplex;


    fn re(&self) -> AnyReal;
    fn im(&self) -> AnyReal;

    fn additive_inverse(&self) -> AnyComplex;
    fn multiplicative_inverse(&self) -> AnyComplex;
    
    fn real_exponent(&self) -> AnyNumber;
}

trait Number {
    fn plus(&self, rhs: &AnyNumber) -> AnyNumber;
    fn subtract(&self, rhs: &AnyNumber) -> AnyNumber;
    fn times(&self, rhs: &AnyNumber) -> AnyNumber;
    fn divide(&self, rhs: &AnyNumber) -> AnyNumber;

    fn neg(&self) -> AnyNumber;
    fn abs(&self) -> AnyNumber;


    fn power(&self, rhs: &AnyNumber) -> AnyNumber;
    fn log(&self, base: &AnyNumber) -> AnyNumber;

    fn compare(&self, rhs: &AnyNumber) -> Option<Ordering>;

    fn number_q(&self) -> bool;
    fn real_q(&self) -> bool;
    fn exact_number_q(&self) -> bool;
    fn integer_q(&self) -> bool;
    fn inexact_number_q(&self) -> bool;
    fn machine_number_q(&self) -> bool;

    fn head(&self) -> Symbol;
    fn round(&self, to: &AnyNumber, kind: Round) -> AnyNumber;
    fn numerical(&self, precision: Option<&AnyNumber>) -> AnyNumber;
    //fn rationalize(&self, dx: Option<f64>) -> AnyRational;
    
    fn precision(&self) -> f64;

    fn conjugate(&self) -> AnyComplex;

    //fn integer_exponent(&self) -> AnyNumber;
    //fn mantissa_exponent(&self) -> Option<(Real, Integer)>;
    //fn numerator_denominator(&self, rhs: AnyNumber) -> AnyNumber;
}

type ComplexBigRational = num::Complex<BigRational>;
type Complex32 = num::Complex<f32>; 
type Complex64 = num::Complex<f64>; 

pub enum Round {
    NearestTiesToEven,
    TowardPositive,
    TowardNegative,
    TowardZero,
    TowardsInfinity,
    NearestTiesToAway,
}


// impl<T : num::Integer+ num::Float> Integer for T {
//     fn gcd(self, other: &AnyInteger) -> AnyInteger {
//         self.gcd(other)
//     }
// }

// impl<T : num::Integer> Rational for T {
//     fn numerator(&self) -> AnyInteger {
//         self
//     }

//     fn denominator(&self) -> AnyInteger {
//         T::one()
//     }
// }

impl<T : num::Float> Real for T {
    fn real_digits(&self, base: &AnyInteger) -> (Vec<AnyInteger>, AnyInteger) {
        todo!()
    }

    fn integer_part(&self) -> AnyInteger {
        todo!()
    }

    fn fractional_part(&self) -> AnyReal {
        todo!()
    }

    fn real_floor(&self) -> AnyInteger {
        Box::new(self.floor())
    }

    fn real_ceil(&self) -> AnyInteger {
        self.ceil()
    }
}

impl<T : num::Float> Complex for T {
    fn re(&self) -> AnyReal {
        self
    }

    fn im(&self) -> AnyReal {
        T::zero()
    }

    fn additive_inverse(&self) -> AnyComplex {
        -self
    }

    fn multiplicative_inverse(&self) -> AnyComplex {
        todo!()
    }

    fn real_exponent(&self) -> AnyNumber {
        todo!()
    }
}


impl<T : num::Num> Number for T {
    fn plus(&self, rhs: &AnyNumber) -> AnyNumber {
        self.add(rhs)
    }

    fn subtract(&self, rhs: &AnyNumber) -> AnyNumber {
        self.sub(rhs)
    }

    fn times(&self, rhs: &AnyNumber) -> AnyNumber {
        todo!()
    }

    fn divide(&self, rhs: &AnyNumber) -> AnyNumber {
        todo!()
    }

    fn neg(&self) -> AnyNumber {
        todo!()
    }

    fn abs(&self) -> AnyNumber {
        todo!()
    }

    fn power(&self, rhs: &AnyNumber) -> AnyNumber {
        todo!()
    }

    fn log(&self, base: &AnyNumber) -> AnyNumber {
        todo!()
    }

    fn compare(&self, rhs: &AnyNumber) -> Option<Ordering> {
        todo!()
    }

    fn number_q(&self) -> bool {
        todo!()
    }

    fn real_q(&self) -> bool {
        todo!()
    }

    fn exact_number_q(&self) -> bool {
        todo!()
    }

    fn integer_q(&self) -> bool {
        todo!()
    }

    fn inexact_number_q(&self) -> bool {
        todo!()
    }

    fn machine_number_q(&self) -> bool {
        todo!()
    }

    fn head(&self) -> Symbol {
        todo!()
    }

    fn round(&self, to: &AnyNumber, kind: Round) -> AnyNumber {
        todo!()
    }

    fn numerical(&self, precision: Option<&AnyNumber>) -> AnyNumber {
        todo!()
    }

    fn precision(&self) -> f64 {
        todo!()
    }

    fn conjugate(&self) -> AnyComplex {
        todo!()
    }
}


/* 
pub enum AnyMachineInteger{
    UnsignedInteger8(u8),
    Integer16(i16),
    UnsignedInteger16(u16),
    Integer32(i32),
    UnsignedInteger32(u32),
    Integer64(i64),
    UnsignedInteger64(u64),
    Integer128(i128),
    UnsignedInteger128(u128),
} 
pub enum AnyInteger {
    Machine(AnyMachineInteger),
    Big(BigInt),
}

pub enum AnyRational {
    Integer(AnyInteger),
    BigRational(BigRational),
    Rational32(Rational32),
    Rational64(Rational64),
}

pub enum AnyMachineReal {
    Real32(f32),
    Real64(f64),
}

pub enum AnyMachineNumber {
    Real(AnyMachineReal),
    Integer(AnyMachineInteger),
    Complex(AnyMachineComplex),
}

pub enum AnyReal {
    Rational(AnyRational),
    Real32(f32),
    Real64(f64),
    Big(BigReal),
}

pub enum AnyMachineComplex {
    Complex32(Complex32),
    Complex64(Complex64),
}
pub enum AnyComplex {
    Real(AnyReal),
    Complex32(Complex32),
    Complex64(Complex64),
    ComplexBig(ComplexBigReal),
    ComplexRational(ComplexBigRational),
}


pub enum AnyNumber {
    Complex(AnyComplex),
    DirectedInfinity(DirectedInfinity),
    Indeterminate,
}

*/