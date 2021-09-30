use std::{collections::HashMap, convert::TryInto, fmt::{Debug, Display}, net::Incoming, path::Ancestors};

use enum_dispatch::enum_dispatch;

struct Definition {}

pub struct Context {
    name: String,
    defintions: HashMap<String, Definition>,
}

#[derive(Debug, Clone)]
pub struct ComplexExpression {
    head: Box<AnyExpression>,
    args: Vec<AnyExpression>,
}

impl ComplexExpression {
    pub fn new(head: Box<AnyExpression>, args: Vec<AnyExpression>) -> Self { Self { head, args } }
}

impl Expression for ComplexExpression {
    fn head(&self) -> AnyExpression {
        *self.head.clone()
    }

    fn length(&self) -> usize {
        self.args.len()
    }

    fn index(&self, index: &Integer) -> Option<&AnyExpression> {
        match index {
            Integer::MachineUnsigned(i) => self.args.get(i-1),
            Integer::Big => None,
        }
    }

    fn eval(&self, context: &Context) -> Option<AnyExpression> {
        None
    }

    fn repr(&self) -> String {
        format!("{}{:?}", self.head.repr(), self.args)
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            name: "Global".to_string(),
            defintions: HashMap::new(),
        }
    }
}


pub enum Real {
    Machine(f64),
    Arbitary,
}

#[derive(Debug, Clone)]
pub enum Integer {
    MachineUnsigned(usize),
    MachineSigned(isize),
    Big,
}

// struct UpTo{
//     value: Integer
// }

// enum SpanIndexParam {
//     All,
//     Integer(usize)
// }

// enum Span {
//     Range(usize, usize),
//     From(usize),
//     To(usize),
//     All,
//     StepRange(usize, usize),
//     StepFrom(usize),
//     StepTo(usize),
//     Step
// }

// enum PartSpec {
//     Index(usize),
//     Span(Span),
//     Key(Box<AnyExpression>),
// }

#[enum_dispatch(AnyExpression)]
pub trait Expression : Debug{
    fn head(&self) -> AnyExpression;
    fn length(&self) -> usize;
    //fn part(&self, part: PartSpec) -> Option<&AnyExpression>;

    fn index(&self, index: &Integer) -> Option<&AnyExpression>;
    fn eval(&self, context: &Context) -> Option<AnyExpression>;

    fn repr(&self) -> String;
}



pub type Symbol = &'static str;
pub type List = Vec<AnyExpression>;

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum AnyExpression {
    Symbol(Symbol),
    List(List),
    Integer(Integer),
    MachineInteger(usize),
    ComplexExpression(ComplexExpression)
}


// struct Symbol<'a>{
//     name: &'a str
// }

// impl PartialEq for Symbol<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.name as *const str == other.name as *const str
//     }
// }

// impl<'a> Symbol<'a> {
//     const fn new(name: &'a str) -> Self { Self { name } }
// }

pub const SYMBOL: Symbol = "Symbol";
pub const INTEGER: Symbol = "Integer";
pub const LIST: Symbol = "List";


impl Expression for List {
    fn head(&self) -> AnyExpression {
        LIST.into()
    }

    fn length(&self) -> usize {
        self.len()
    }

    fn index(&self, index: &Integer) -> Option<&AnyExpression> {
        match index {
            Integer::MachineUnsigned(i) => self.get(*i),
            Integer::Big => None,
        }
        
    }

    fn eval(&self, _context: &Context) -> Option<AnyExpression> {
        None
    }

    fn repr(&self) -> String {
        format!("List{:?}", self)
    }
}

//const SYMBOL_SYMBOL:Symbol = Symbol::new("Symbol");

impl Expression for Symbol {
    fn head(&self) -> AnyExpression {
        SYMBOL.into()
    }

    fn length(&self) -> usize {
        0
    }

    fn index(&self, _index: &Integer) -> Option<&AnyExpression> {
        None
    }

    fn eval(&self, _context: &Context) -> Option<AnyExpression> {
        None
    }

    fn repr(&self) -> String {
        self.to_string()
    }
}

impl Expression for Integer {
    fn head(&self) -> AnyExpression {
        INTEGER.into()
    }

    fn length(&self) -> usize {
        0
    }

    fn index(&self, _index: &Integer) -> Option<&AnyExpression> {
        None
    }

    fn eval(&self, _context: &Context) -> Option<AnyExpression> {
        None
    }

    fn repr(&self) -> String {
        match self{
            Integer::MachineUnsigned(v) => format!("{}", v),
            Integer::Big => "BigInteger[]".to_string(),
        }
    }
}

impl Expression for usize {
    fn head(&self) -> AnyExpression {
        INTEGER.into()
    }

    fn length(&self) -> usize {
        0
    }

    fn index(&self, _index: &Integer) -> Option<&AnyExpression> {
        None
    }

    fn eval(&self, _context: &Context) -> Option<AnyExpression> {
        Some(AnyExpression::Integer(Integer::MachineUnsigned(*self)))
    }

    fn repr(&self) -> String {
        format!("raw {}", self)
    }
}
