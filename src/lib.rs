#[macro_use]
extern crate quickcheck;

mod number;
mod numeric_array;

pub use crate::number::{MachineFloat, MachineInt, Number};
pub use crate::numeric_array::NumericArray;

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::{collections::HashMap, ops::Index};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Symbol,

    Number,
    Boolean,
    String,

    Null,

    Missing,

    List,
    Association,

    NumericArray,
    SparseArray,

    Named(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Normal {
    Null,
    Boolean(bool),
    Number(Number),
    String(String),
    List(Vec<Normal>),
    Dict(HashMap<String, Normal>),
}

pub struct SparseArray {
    size: Vec<MachineInt>,
    default: Box<Expression>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Expression {
    Null,
    Symbol(Symbol),
    Number(Number),
    Boolean(bool),
    String(String),
    Compound(Symbol, Vec<Expression>, HashMap<Symbol, Expression>),
    Complex {
        head: Box<Expression>,
        body: Vec<Expression>,
    },
    Association(HashMap<Symbol, Expression>),
    List(Vec<Expression>),
    NumericArray(NumericArray),
}

pub fn head(expr: Expression) -> Expression {
    match expr {
        Expression::Null => Expression::Symbol(Symbol::Null),
        Expression::Symbol(_) => Expression::Symbol(Symbol::Symbol),

        Expression::Complex { head, body } => todo!(),
        Expression::Association(_) => todo!(),
        Expression::List(_) => Expression::Symbol(Symbol::List),
        Expression::Number(_) => Expression::Symbol(Symbol::Number),
        Expression::Boolean(_) => Expression::Symbol(Symbol::Boolean),
        Expression::String(_) => Expression::Symbol(Symbol::String),
        Expression::NumericArray(_) => todo!(),
        Expression::Compound(_, _, _) => todo!(),
    }
}

pub fn get(from: Expression, key: Expression) {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
