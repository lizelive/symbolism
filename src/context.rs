use std::{collections::HashMap, sync::Arc};

struct Definition {
    
}

struct Context{
    name: String,
    defintions:HashMap<String, Definition>
}

enum Integer {
    Machine(isize),
    Big,
}

struct UpTo{

}



enum SpanIndexParam {
    All,
    Integer(isize)
}

enum Span {
    Range(isize, isize),
    From(isize),
    To(isize),
    All,
    StepRange(isize, isize),
    StepFrom(isize),
    StepTo(isize),
    Step
}

enum PartSpec {
    Index(usize),
    Span(Span),
    Key(Box<dyn Expression>),
}

trait Expression {
    fn head(&self) -> &dyn Expression;
    fn length(&self) -> usize;
    //fn part(&self, part: PartSpec) -> Option<&dyn Expression>;
    
    fn index(&self, index: &Integer) -> Option<&dyn Expression>;
    fn eval(&self, context: &Context) -> Option<Box<dyn Expression>>;
}


struct Symbol<'a>{
    name: &'a str
}

impl PartialEq for Symbol<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name as *const str == other.name as *const str
    }
}

impl<'a> Symbol<'a> {
    const fn new(name: &'a str) -> Self { Self { name } }
}

const SYMBOL: &str = "Symbol";
const SYMBOL_SYMBOL:Symbol = Symbol::new("Symbol");

impl Expression for &str {
    fn head(&self) -> &dyn Expression {
        &SYMBOL
    }

    fn length(&self) -> usize {
        0
    }

    fn index(&self, index: &Integer) -> Option<&dyn Expression> {
        match index {
            Integer::Machine(0) => Some(&SYMBOL),
            _ => None,
        }
    }

    fn eval(&self, context: &Context) -> Option<Box<dyn Expression>> {
        None
    }
}

