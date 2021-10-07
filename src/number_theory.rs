use num::Integer;

//use crate::context::Integer;

fn factor_integer(){
    todo!()
}


fn extended_gcd<T: Integer>(a: T, b: T)-> T{
    let mut s: T = T::zero();
    let mut old_s:T = T::one();
    let mut  r = b;
    let mut old_r = a;
         
    while (! r.is_zero()){
        let quotient = old_r / r;
        let (old_r, r) = (r, old_r - quotient * r);
        let (old_s, s) = (s, old_s - quotient * s);
    }
    if (b.is_zero()){
        bezout_t = quotient(old_r − old_s × a, b)
    }
    else
        bezout_t := 0
    
    output "Bézout coefficients:", (old_s, bezout_t)
    output "greatest common divisor:", old_r
}