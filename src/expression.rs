use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
    sync::Arc,
};

use serde::{Deserialize, Serialize};

use crate::{pattern::Form, symbol::BuiltinSymbol};

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

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Definition {
//     symbol: Symbol,
//     downvalues: Vec<Expression>,
//     upvalues: Vec<Expression>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Context {
//     name: String,
//     definitions: Vec<Definition>,
//     parent: Option<Box<Context>>,
// }

// impl Context {
//     pub fn new(name: String) -> Self {
//         Self {
//             name,
//             definitions: Vec::new(),
//             parent: None,
//         }
//     }
//     fn get(&self, index: String) -> Option<&Definition> {
//         for def in self.definitions {
//             if def.symbol.name() == index {
//                 return Some(&def);
//             }
//         }
//         return None;
//     }
//     fn get_or_make(&mut self, name: String) -> &Definition {
//         if let Some(def) = self.get(name) {
//             def
//         } else {
//             let symbol = Symbol::get(name);
//             let def: Definition = Definition {
//                 downvalues: Vec::new(),
//                 upvalues: Vec::new(),
//                 symbol,
//             };
//             self.definitions.push(def);
//             &self.get(name).expect("i just set it")
//         }
//     }
// }

// static mut GLOBAL: Box<Context> = Box::new(Context::new("Global".to_string()));

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BasicExpression {
    Symbol { value: BuiltinSymbol },
    Real { value: f64 },
    Integer { value: i64 },
    String { value: String },
}

impl From<String> for BasicExpression {
    fn from(value: String) -> Self {
        Self::String { value }
    }
}

impl From<f64> for BasicExpression {
    fn from(value: f64) -> Self {
        Self::Real { value }
    }
}

impl From<i64> for BasicExpression {
    fn from(value: i64) -> Self {
        Self::Integer { value }
    }
}

impl BasicExpression {
    fn repr(&self) -> String {
        match self {
            BasicExpression::Symbol { value } => value.name(),
            BasicExpression::Real { value } => value.to_string(),
            BasicExpression::Integer { value } => value.to_string(),
            BasicExpression::String { value } => format!(r#""{}""#, value),
        }
    }

    fn head(&self) -> Expression {
        Expression::get_symbol(match self {
            BasicExpression::Symbol { value: _ } => BuiltinSymbol::Symbol,
            BasicExpression::Real { value: _ } => BuiltinSymbol::Real,
            BasicExpression::Integer { value: _ } => BuiltinSymbol::Integer,
            BasicExpression::String { value: _ } => BuiltinSymbol::String,
        })
    }
}

impl PartialEq for BasicExpression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Symbol { value: l_name }, Self::Symbol { value: r_name }) => l_name == r_name,
            (Self::Real { value: l_value }, Self::Real { value: r_value }) => l_value == r_value,
            (Self::Integer { value: l_value }, Self::Integer { value: r_value }) => {
                l_value == r_value
            }
            (Self::String { value: l_value }, Self::String { value: r_value }) => {
                l_value == r_value
            }
            _ => false,
        }
    }
}

impl Eq for BasicExpression {}

impl std::hash::Hash for BasicExpression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct ComplexExpression {
    head: Box<Expression>,
    args: Vec<Expression>,
    options: HashMap<BasicExpression, Expression>,
}

impl ComplexExpression {
    fn repr(&self) -> String {
        let args_str = self.args.iter().map(Expression::repr);
        let rules_str = self
            .options
            .iter()
            .map(|(key, value)| format!("{}->{}", key.repr(), value.repr()));
        let paramaters: Vec<String> = args_str.chain(rules_str).collect(); // oh well.
        let paramaters = paramaters.join(", ");
        format!("{}[{}]", self.head.repr(), paramaters)
    }
}

impl PartialEq for ComplexExpression {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head && self.args == other.args && self.options == other.options
    }
}

impl std::hash::Hash for ComplexExpression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.head.hash(state);
        self.args.hash(state);
        //self.rules.hash(state);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Pattern(Form),
    Basic(BasicExpression),
    Complex(ComplexExpression),
}

impl From<ComplexExpression> for Expression {
    fn from(v: ComplexExpression) -> Self {
        Self::Complex(v)
    }
}

impl From<BasicExpression> for Expression {
    fn from(v: BasicExpression) -> Self {
        Self::Basic(v)
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self::Basic(value.into())
    }
}

impl From<f64> for Expression {
    fn from(value: f64) -> Self {
        Self::Basic(value.into())
    }
}

impl From<i64> for Expression {
    fn from(value: i64) -> Self {
        Self::Basic(value.into())
    }
}

impl Expression {
    pub fn new(head: BuiltinSymbol, args: Vec<Expression>) -> Expression {
        Expression::Complex(ComplexExpression {
            head: Box::new(Expression::get_symbol(head).clone()),
            args,
            options: HashMap::new(),
        })
    }

    pub fn length(&self) -> usize {
        match self {
            Expression::Pattern(_) => todo!(),
            Expression::Basic(_) => 0,
            Expression::Complex(expr) => expr.args.len() + expr.options.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&Expression> {
        if index == 0 {
            Some(self.head())
        } else {
            let len = self.length();
            match self {
                Expression::Pattern(_) => todo!(),
                Expression::Basic(_) => None,
                Expression::Complex(expr) => Some(&expr.args[index]),
            }
        }
    }

    pub fn repr(&self) -> String {
        match self {
            Expression::Basic(expr) => expr.repr(),
            Expression::Complex(expr) => expr.repr(),
            Expression::Pattern(_) => todo!(),
        }
    }
    pub fn head(&self) -> Expression {
        match self {
            Expression::Basic(expr) => expr.head(),
            Expression::Complex(expr) => *expr.head,
            Expression::Pattern(pat) => pat.head(),
        }
    }

    fn get_symbol(symbol: BuiltinSymbol) -> Expression {
        symbol.into()
    }

    pub(crate) fn iter(&self) -> Vec<Expression>{
        let out:Vec<Expression> = match self {
            Expression::Pattern(_) => todo!(),
            Expression::Basic(_) => todo!(),
            Expression::Complex(expr) => expr.args.iter().chain(
                expr.options
                    .iter()
                    .map(|(k, v)| Expression::new(BuiltinSymbol::Rule, vec![
                        *v
                    ])),
            ).collect(),
        }
        return out
    }
}
