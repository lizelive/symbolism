use std::{
    ops::{Add, Mul, Div, Sub},
};

pub type MachineFloat = f64;
pub type MachineInt = i64;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Number {
    MachineFloat(MachineFloat),
    MachineInt(MachineInt),
    //MachineComplex(f64, f64)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumberRef<'a> {
    MachineFloat(MachineFloat),
    MachineInt(MachineInt),
    RefMachineFloat(&'a MachineFloat),
    RefMachineInt(&'a MachineInt),
    //MachineComplex(f64, f64)
}

impl From<MachineFloat> for Number {
    fn from(v: MachineFloat) -> Self {
        Self::MachineFloat(v)
    }
}

impl From<Number> for MachineFloat {
    fn from(val: Number) -> Self {
        match val {
            Number::MachineFloat(v) => v as MachineFloat,
            Number::MachineInt(v) => v as MachineFloat,
        }
    }
}

impl From<Number> for MachineInt {
    fn from(val: Number) -> Self {
        match val {
            Number::MachineFloat(v) => v as MachineInt,
            Number::MachineInt(v) => v as MachineInt,
        }
    }
}

impl From<MachineInt> for Number {
    fn from(val: MachineInt) -> Self {
        Number::MachineInt(val)
    }
}

impl Add<Number> for Number {
    type Output = Self;

    fn add(self, rhs: Number) -> Self::Output {
        match self {
            Number::MachineFloat(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar + br),
                Number::MachineInt(br) => Number::MachineFloat(ar + br as MachineFloat),
            },
            Number::MachineInt(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar as MachineFloat + br),
                Number::MachineInt(br) => Number::MachineInt(ar + br),
            },
        }
    }
}

impl Mul<Number> for Number {
    type Output = Self;

    fn mul(self, rhs: Number) -> Self::Output {
        match self {
            Number::MachineFloat(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar * br),
                Number::MachineInt(br) => Number::MachineFloat(ar * br as MachineFloat),
            },
            Number::MachineInt(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar as MachineFloat * br),
                Number::MachineInt(br) => Number::MachineInt(ar * br),
            },
        }
    }
}

impl Div<Number> for Number {
    type Output = Self;

    fn div(self, rhs: Number) -> Self::Output {
        match self {
            Number::MachineFloat(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar / br),
                Number::MachineInt(br) => Number::MachineFloat(ar / br as MachineFloat),
            },
            Number::MachineInt(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar as MachineFloat / br),
                Number::MachineInt(br) => Number::MachineInt(ar / br),
            },
        }
    }
}

impl Sub<Number> for Number {
    type Output = Self;

    fn sub(self, rhs: Number) -> Self::Output {
        match self {
            Number::MachineFloat(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar - br),
                Number::MachineInt(br) => Number::MachineFloat(ar - br as MachineFloat),
            },
            Number::MachineInt(ar) => match rhs {
                Number::MachineFloat(br) => Number::MachineFloat(ar as MachineFloat - br),
                Number::MachineInt(br) => Number::MachineInt(ar - br),
            },
        }
    }
}