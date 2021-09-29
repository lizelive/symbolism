mod expression;
mod pattern;
mod attributes;
mod symbol;

mod context;

pub use  crate::expression::Expression;
pub use  crate::symbol::BuiltinSymbol;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{Expression, expression::BasicExpression};
    #[test]
    fn repr_int() {
        let expr: Expression = 10.into();
        assert_eq!(expr.repr(), "10");
    }

    #[test]
    fn repr_plus() {
        let i10: Expression = 10.into();
        let f1_23: Expression = 1.23.into();

        let expr= Expression::new("Plus", vec![i10, f1_23]);
        assert_eq!(expr.repr(), "Plus[10, 1.23]");
    }
}
