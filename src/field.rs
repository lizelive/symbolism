use std::ops::{Add, Div, Mul, Sub};

trait Field : Sized + Add + Sub + Mul + Div + Default {
    fn additive_inverse(self) -> Self;
    fn multiplicative_inverse(self) -> Self;

    fn additive_identity() -> Self;
    fn multiplicative_identity() -> Self;
}

trait VectorSpace : Sized + Add + Sub + Mul + Div + Default {
    fn additive_inverse(self) -> Self;
    fn multiplicative_inverse(self) -> Self;
    
    fn additive_identity() -> Self;
    fn multiplicative_identity() -> Self;
}