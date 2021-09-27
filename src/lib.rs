#[macro_use]
extern crate quickcheck;

mod number;
mod numeric_array;

pub use crate::number::{MachineFloat, MachineInt, Number};
pub use crate::numeric_array::NumericArray;

use serde::{Deserialize, Serialize};
use serde_json::value;
use std::borrow::Borrow;
use std::{collections::HashMap, ops::Index};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    Symbol,

    Number,
    Boolean,
    String,

    Function,

    Null,

    Missing,

    List,
    Association,

    Set,
    Hold,    

    NumericArray,

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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Primitive {
    Boolean(bool),
    Real(MachineFloat),
    Integer(MachineInt),
    String(String),
}
pub enum PrimitiveType {
    Boolean,
    Real,
    Integer,
    Complex,
    String,
}

struct Function{
    arguments: HashMap<Symbol, Expression>,
    body: Expression,
}

struct NativeFunction {

}

struct Context {
    data: HashMap<Symbol, Expression>
}

impl Context {
    fn set(&mut self, symbol: Symbol, value: Expression) {
        self.data.insert(symbol, value);
    }
    fn get(&self, symbol: Symbol) -> Option<Expression> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Expression {
    Null,
    Symbol(Symbol),
    
    Number(Number),

    Boolean(bool),
    String(String),

    Compound(Symbol, Vec<Expression>, HashMap<Symbol, Expression>),
    Slot(MachineInt),
    Complex {
        head: Box<Expression>,
        body: Vec<Expression>,
    },
    Set(Symbol, Box<Expression>),
    Hold(Box<Expression>),
    Function { arguments: HashMap<Symbol, Expression>, body: Box<Expression> },
    Association(HashMap<Symbol, Expression>),
    List(Vec<Expression>),
    NumericArray(NumericArray),
}

impl From<Symbol> for Expression {
    fn from(v: Symbol) -> Self {
        Self::Symbol(v)
    }
}

impl From<Vec<Expression>> for Expression {
    fn from(v: Vec<Expression>) -> Self {
        Self::List(v)
    }
}

impl From<Number> for Expression {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}
impl Expression {
    pub fn head(&self) -> Expression {
        match self {
            Expression::Null => Symbol::Null.into(),
            Expression::Symbol(_) => Symbol::Symbol.into(),
    
            Expression::Complex { head, body } => todo!(),
            Expression::Association(_) => Symbol::Association.into(),
            Expression::List(_) => Symbol::List.into(),
            Expression::Number(_) => Symbol::Number.into(),
            Expression::Boolean(_) => Symbol::Boolean.into(),
            Expression::String(_) => Symbol::String.into(),
            Expression::NumericArray(_) => Symbol::NumericArray.into(),
            Expression::Compound(head, _, _) => head.clone().into(),
            
            Expression::Slot(_) => todo!(),

            Expression::Function { arguments, body } => todo!(),
            Expression::Set(_, _) => Symbol::Set.into(),
            Expression::Hold(_) => Symbol::Hold.into(),
        }
    }

    pub fn evaluate_end(self, mut ctx: &Context) -> Expression {
        match self.evaluate(ctx){
            Some(expression) => expression,
            None => self,
        }
    }
    pub fn evaluate(&self, mut ctx: &Context) -> Option<Expression> {
        let head_head = self.head().head();

        match self{
            Expression::Compound(head, _, _) => todo!(),
            Expression::Complex { head, body } => todo!(),
            Expression::Function { arguments, body } => todo!(),
            Expression::Set(symbol , expression) => {
                let expr = expression.as_ref();
                match self.evaluate(ctx){
                    Some(value) => ctx.set(symbol.clone(), value),
                    None => ctx.set(symbol.clone(), expr.clone()),
                };
                let value = expr.evaluate_end(ctx);
                ctx.set(symbol.clone(), value);
                Some(Expression::Null)
            },
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
