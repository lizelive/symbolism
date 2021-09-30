use std::{cmp::Ordering, ops::{Add, Mul, Div, Sub}};

use num::{BigInt, BigRational, Num, Rational32, Rational64};

use crate::context::Symbol;


enum Comparison {
    
}

fn add<TL, TR>(lhs:&TL, rhs: &TR) -> AnyNumber where TR: Number, TL : Number{
    lhs.add(rhs)
}

trait Number {
    fn add(&self, rhs: &AnyNumber) -> AnyNumber;
    fn sub(&self, rhs: &AnyNumber) -> AnyNumber;
    fn mul(&self, rhs: &AnyNumber) -> AnyNumber;
    fn div(&self, rhs: &AnyNumber) -> AnyNumber;

    fn neg(&self) -> AnyNumber;

    fn abs(&self) -> AnyNumber;

    fn power(&self, rhs: &AnyNumber) -> AnyNumber;
    fn log(&self, base: &AnyNumber) -> AnyNumber;

    fn compare(&self, rhs: &AnyNumber) -> Option<Ordering>;

    fn real_q(&self) -> bool;
    fn exact_number_q(&self) -> bool;
    fn integer_q(&self) -> bool;
    
    fn head(&self) -> Symbol;
    fn round(&self, to: &AnyNumber, kind: Round) -> AnyNumber;
    fn numerical(&self, precision: Option<f64>) -> AnyNumber;
    fn rationalize(&self, dx: Option<f64>) -> AnyNumber;
    
    fn machine_real_number(&self) -> f64;
    
    fn inexact_number_q(&self) -> bool;
    fn machine_number_q(&self) -> bool;
    fn precision(&self) -> f64;
    fn real_exponent(&self) -> AnyNumber;
    //fn integer_exponent(&self) -> AnyNumber;
    //fn mantissa_exponent(&self) -> Option<(Real, Integer)>;
    //fn numerator_denominator(&self, rhs: AnyNumber) -> AnyNumber;
}




type BigReal = f64;

type ComplexBigRational = num::Complex<BigRational>;
type Complex32 = num::Complex<f32>; 
type Complex64 = num::Complex<f64>; 
type ComplexBigReal = num::Complex<BigReal>;


pub enum Round {
    NearestTiesToEven,
    TowardPositive,
    TowardNegative,
    TowardZero,
    TowardsInfinity,
    NearestTiesToAway,
}

pub enum AnyInteger {
    Integer8(i8),
    UnsignedInteger8(u8),
    Integer16(i16),
    UnsignedInteger16(u16),
    Integer32(i32),
    UnsignedInteger32(u32),
    Integer64(i64),
    UnsignedInteger64(u64),
    Integer128(i128),
    UnsignedInteger128(u128),
    BigInt(BigInt),
}

pub enum AnyReal {
    Real32(f32),
    Real64(f64),
    BigReal(BigReal),
    Overflow,
    Underflow,
}

pub enum AnyRational {
    BigRational(BigRational),
    Rational32(Rational32),
    Rational64(Rational64),
}

pub enum Complex {
    Real32(Complex32),
    Real64(Complex64),
    BigReal(ComplexBigReal),
    BigRational(ComplexBigRational),
}

pub enum DirectedInfinity {
    Real(AnyReal),
    Integer(AnyInteger),
    Complex,
}

pub enum AnyNumber {
    Real(AnyReal),
    Rational(AnyRational),
    Complex(Complex),
    DirectedInfinity(DirectedInfinity),

    Indeterminate,
}

pub enum AnyPrimitive {
    Symbol,
    Boolean(bool),
    Number(AnyNumber),
}

pub enum AnyExpression{
    Primitive(AnyPrimitive),
    NumericArray,
    List,
    Assocation,
    External,
    ComplexExpression,
}