
extern crate enum_dispatch;

mod context;
mod number;
mod bigreal;
mod atomic;
mod ast;
mod field;
mod number_theory;
mod curry;
mod nest;
mod list;
mod number;

use context::*;

macro_rules! symbol {
    ($x:ident) => {{
        const SYMBOL: Symbol = stringify!($x);
        SYMBOL
    }
    };
}

macro_rules! expr {
    ({$( $x:expr ),* }) => {{
        list!($($x),+)
    }};
    ($head:ident) => {
        symbol!(ident)
    };
    ($head:ident[$( $a:expr ),*]) => {{
        let head:AnyExpression = symbol!($head).into();
        ComplexExpression::new(
            Box::new(head),
            list!($($a),+).into(),
        )
    }};
    ($( $x:expr );*) => {{
        list!($($x),+)
    }};
    ($x:expr) => {{
        list!($($x),+)
    }};
}

macro_rules! list {
    ($( $x:expr ),* ) => {
        {
            let list: List = vec![
                $(
                    $x.into(),
                )*
            ];
            //let expr: AnyExpression = list.into();
            list
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn repr_int() {
        let ctx = Context::new();
        let expr = expr!(help[1]);
        expr.eval(&ctx);
        let expr = AnyExpression::from(expr);
        println!("{:?} == {}", expr, expr.repr())
    }

    // fn repr_plus() {
    //     let i10: Expression = 10.into();
    //     let f1_23: Expression = 1.23.into();

    //     let expr= Expression::new("Plus", vec![i10, f1_23]);
    //     assert_eq!(expr.repr(), "Plus[10, 1.23]");
    // }
}
