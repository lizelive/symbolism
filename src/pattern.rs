use std::collections::HashMap;

use crate::{BuiltinSymbol, Expression};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Form {
    // basic
    Blank {
        head: Option<BuiltinSymbol>,
    },
    BlankSequence {
        head: Option<BuiltinSymbol>,
    },
    BlankNullSequence {
        head: Option<BuiltinSymbol>,
    },

    // composite
    Alternatives {
        patterns: Vec<Form>,
    },

    Repeated {
        pattern: Box<Form>,
    },

    RepeatedNull {
        patterns: Box<Form>,
    },

    Pattern {
        sym: BuiltinSymbol,
        patten: Box<Form>,
    },

    Except {
        pattern: Box<Form>,
    },

    Longest {
        pattern: Box<Form>,
    },

    Shortest {
        pattern: Box<Form>,
    },

    OptionsPattern {
        defaults: Option<Box<Expression>>,
    },

    PatternSequence {
        patterns: Vec<Form>,
    },

    Verbatim {
        expr: Box<Expression>,
    },

    HoldPattern {
        expr: Box<Expression>,
    },

    OrderlessPatternSequence {
        patterns: Vec<Form>,
    },

    KeyValuePattern {
        patterns: Vec<Form>,
    },

    // restrctions
    Condition {
        pattern: Box<Form>,
        test: Box<Expression>,
    },
    PatternTest {
        pattern: Box<Form>,
        test: Box<Expression>,
    },

    // pattern defaults
    Optional {
        pattern: Box<Form>,
        default: Option<Box<Expression>>,
    },
    // StringExpression {
    //     pattern: StringExpression,
    // },
}

fn match_q(expr: &Expression, form: &Form) -> bool {
    match form {
        Form::Blank { head } => {
            if let Some(head) = head {
                expr.head() == head.clone().into()
            } else {
                true
            }
        }
        Form::BlankSequence { head } => {
            if expr.length() == 0 {
                false
            } else if let Some(head) = head {
                expr.iter()
            } else {
                true
            }
        }
        Form::BlankNullSequence { head } => {
            if let Some(head) = head {
                expr.head() == head.clone().into()
            } else {
                true
            }
        }
        Form::Alternatives { patterns } => todo!(),
        Form::Repeated { pattern } => todo!(),
        Form::RepeatedNull { patterns } => todo!(),
        Form::Pattern { sym, patten } => todo!(),
        Form::Except { pattern } => todo!(),
        Form::Longest { pattern } => todo!(),
        Form::Shortest { pattern } => todo!(),
        Form::OptionsPattern { defaults } => todo!(),
        Form::PatternSequence { patterns } => todo!(),
        Form::Verbatim { expr } => todo!(),
        Form::HoldPattern { expr } => todo!(),
        Form::OrderlessPatternSequence { patterns } => todo!(),
        Form::KeyValuePattern { patterns } => todo!(),
        Form::Condition { pattern, test } => todo!(),
        Form::PatternTest { pattern, test } => todo!(),
        Form::Optional { pattern, default } => todo!(),
    }
}
impl Form {


pub fn head(&self) -> Expression {
    match self {
        Form::Blank { head } => BuiltinSymbol::Blank,
        Form::BlankSequence { head } => BuiltinSymbol::BlankSequence,
        Form::BlankNullSequence { head } => BuiltinSymbol::BlankNullSequence,
        Form::Alternatives { patterns } => BuiltinSymbol::Alternatives,
        Form::Repeated { pattern } => BuiltinSymbol::Repeated,
        Form::RepeatedNull { patterns } => BuiltinSymbol::RepeatedNull,
        Form::Pattern { sym, patten } => BuiltinSymbol::Pattern,
        Form::Except { pattern } => BuiltinSymbol::Except,
        Form::Longest { pattern } => BuiltinSymbol::Longest,
        Form::Shortest { pattern } => BuiltinSymbol::Shortest,
        Form::OptionsPattern { defaults } => BuiltinSymbol::OptionsPattern,
        Form::PatternSequence { patterns } => BuiltinSymbol::PatternSequence,
        Form::Verbatim { expr } => BuiltinSymbol::Verbatim,
        Form::HoldPattern { expr } => BuiltinSymbol::HoldPattern,
        Form::OrderlessPatternSequence { patterns } => BuiltinSymbol::OrderlessPatternSequence,
        Form::KeyValuePattern { patterns } => BuiltinSymbol::KeyValuePattern,
        Form::Condition { pattern, test } => BuiltinSymbol::Condition,
        Form::PatternTest { pattern, test } => BuiltinSymbol::PatternTest,
        Form::Optional { pattern, default } => BuiltinSymbol::Optional,
    }.into()
}
}
