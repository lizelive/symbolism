use std::{
    ops::{Add, Mul, Div, Sub},
};

pub type MachineFloat = f64;
pub type MachineInt = i64;
pub type MachineComplexFloat = Complex<MachineFloat>;
pub type MachineComplexInt = Complex<MachineInt>;

use nalgebra::Complex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Number {
    MachineFloat(MachineFloat),
    MachineInt(MachineInt),
    MachineComplexFloat(MachineComplexFloat),
    MachineComplexInt(MachineComplexInt),
}

impl From<MachineFloat> for Number {
    fn from(v: MachineFloat) -> Self {
        Self::MachineFloat(v)
    }
}

impl From<MachineInt> for Number {
    fn from(val: MachineInt) -> Self {
        Number::MachineInt(val)
    }
}

impl Into<MachineFloat> for Number {
    fn into(self) -> MachineFloat {
        match self {
            Number::MachineFloat(a) => a as MachineFloat,
            Number::MachineInt(a) => a as MachineFloat,
            Number::MachineComplexFloat(a) => a.re as MachineFloat,
            Number::MachineComplexInt(a) => a.re as MachineFloat,
        }
    }
}

impl From<Number> for MachineInt {
    fn from(val: Number) -> Self {
        match val {
            Number::MachineFloat(a) => a as MachineInt,
            Number::MachineInt(a) => a as MachineInt,
            Number::MachineComplexFloat(a) => a.re as MachineInt,
            Number::MachineComplexInt(a) => a.re as MachineInt,
        }
    }
}

impl Into<MachineComplexInt> for Number {
    fn into(self) -> MachineComplexInt {
        match self {
            Number::MachineFloat(a) => MachineComplexInt::new(a as MachineInt, 0 as MachineInt),
            Number::MachineInt(a) => MachineComplexInt::new(a as MachineInt, 0 as MachineInt),
            Number::MachineComplexFloat(a) => MachineComplexInt::new(a.re as MachineInt, a.im as MachineInt),
            Number::MachineComplexInt(a) => MachineComplexInt::new(a.re as MachineInt, a.im as MachineInt),
        }
    }
}


impl Into<MachineComplexFloat> for Number {
    fn into(self) -> MachineComplexFloat {
        match self {
            Number::MachineFloat(a) => MachineComplexFloat::new(a as MachineFloat, 0 as MachineFloat),
            Number::MachineInt(a) => MachineComplexFloat::new(a as MachineFloat, 0 as MachineFloat),
            Number::MachineComplexFloat(a) => MachineComplexFloat::new(a.re as MachineFloat, a.im as MachineFloat),
            Number::MachineComplexInt(a) => MachineComplexFloat::new(a.re as MachineFloat, a.im as MachineFloat),
        }
    }
}

impl Number {
    fn re(&self) -> Number {
        match self {
            Number::MachineFloat(re) => Number::MachineFloat(*re),
            Number::MachineInt(re) => Number::MachineInt(*re),
            Number::MachineComplexFloat(c) => Number::MachineFloat(c.re),
            Number::MachineComplexInt(c) => Number::MachineInt(c.re),

        }
    }

    fn im(&self) -> Number {
        match self {
            Number::MachineComplexFloat(c) => Number::MachineFloat(c.im),
            Number::MachineComplexInt(c) => Number::MachineInt(c.im),
            Number::MachineFloat(_) => Number::MachineInt(0),
            Number::MachineInt(_) => Number::MachineInt(0),
        }
    }
    
    fn is_complex(&self) -> bool {
        match self{
            Number::MachineFloat(_) => false,
            Number::MachineInt(_) => false,
            Number::MachineComplexFloat(_) => true,
            Number::MachineComplexInt(_) => true,
        }
    }
    
    fn is_integer(&self) -> bool{
        match self{
            Number::MachineFloat(_) => false,
            Number::MachineInt(_) => true,
            Number::MachineComplexFloat(_) => false,
            Number::MachineComplexInt(_) => true,
        }
    }
}

impl Sub<Number> for Number {
    type Output = Number;

    fn sub(self, rhs: Number) -> Self::Output {
        let is_complex = self.is_complex() || rhs.is_complex();
        let is_int = self.is_integer() && rhs.is_integer();
        match (is_complex, is_int){
            (true, true) => {
                let a : MachineComplexInt = self.into();
                let b : MachineComplexInt = rhs.into(); 
                Number::MachineComplexInt(a - b)
            },
            (true, false) => {
                let a : MachineComplexFloat = self.into();
                let b : MachineComplexFloat = rhs.into(); 
                Number::MachineComplexFloat(a - b)
            },
            (false, true) => {
                let a : MachineInt = self.into();
                let b : MachineInt = rhs.into(); 
                Number::MachineInt(a - b)
            },
            (false, false) => {
                let a : MachineFloat = self.into();
                let b : MachineFloat = rhs.into(); 
                Number::MachineFloat(a - b)
            },
        }
    }
}
