#![feature(slice_index_methods)]

use core::slice;
use std::cell::RefCell;
use std::{iter::FromIterator, process::Output, slice::Iter, sync::Arc};

enum LengthSpecifier {
    Exactly(usize),
    UpTo(usize),
    All,
}

enum SequenceSpecification {
    All,
    None,
    First(usize),
    UpTo(usize),
    Last(usize),
    Only(usize),
    Between(usize, usize),
    BetweenBy(usize, usize, usize),
}

trait ListIndex<L>: Sized {
    type Output: Sized;
    fn get(self, list: &L) -> Option<&Self::Output>;
    fn set(self, list: &mut L, value: Self::Output) -> Result<(), OutOfBounds>;

    // fn get_mut(self, list: &mut L) -> Option<&mut Self::Output>;
}

pub struct OutOfBounds;

trait List<T>: Sized
where
    T: Sized,
{
    #[inline]
    fn extract<I: ListIndex<Self>>(&self, index: I) -> Option<&I::Output> {
        index.get(self)
    }

    #[inline]
    fn set<I: ListIndex<Self>>(&mut self, index: I, value: I::Output) -> Result<(), OutOfBounds> {
        index.set(self, value);
        Ok(())
    }

    fn replace_part<I: ListIndex<Self>>(&self, index: I, value: I::Output) -> Result<Self, OutOfBounds> {
        //index.set(self, value);
        let mut new = *self;
        Ok(new)
    }

    fn delete<I: ListIndex<Self>>(&self, index: I) -> Result<Self, OutOfBounds> {
        //index.set(self, value);
        let mut new = *self;
        Ok(new)
    }

    fn prepend<I: ListIndex<Self>>(&self, value: I::Output) -> Result<Self, OutOfBounds> {
        //index.set(self, value);
        let mut new = *self;
        Ok(new)
    }

    fn insert<I: ListIndex<Self>>(&self, index: I, value: I::Output) -> Result<Self, OutOfBounds> {
        //index.set(self, value);
        let mut new = *self;
        Ok(new)
    }

    fn append<I: ListIndex<Self>>(&self, value: I::Output) -> Self{
        //index.set(self, value);
        let mut new = *self;
        new
    }

    fn riffle(&self, value: T) -> Self {
        *self
    }

    fn length(&self) -> usize;
}

impl<T> List<T> for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T, I> ListIndex<Vec<T>> for I
where
    I: slice::SliceIndex<[T]>,
    I::Output: Copy,
{
    type Output = I::Output;

    #[inline]
    fn get(self, list: &Vec<T>) -> Option<&Self::Output> {
        list.extract(self)
    }

    fn set(self, list: &mut Vec<T>, value: Self::Output) -> Result<(), OutOfBounds>{
        if let Some(slice) = list.get_mut(self) {
            *slice = value;
            Ok(())
        }
        else {
            Err(OutOfBounds)
        }
    }

    // #[inline]
    // fn get_mut(self, l: &mut Vec<T>) -> Option<&mut Self::Output>{
    //     (self as I).get_mut(l)
    // }
}

fn nest<B, F>(mut f: F, init: B, n: usize) -> B
where
    B: Sized,
    F: FnMut(B) -> B,
{
    let mut accum = init;
    for _ in 1..n {
        accum = f(accum);
    }
    accum
}

#[cfg(test)]
mod tests {
    use crate::list::{nest, List};

    #[test]
    fn it_works() {
        let v = vec![1, 2];
        assert_eq!(v.extract(1), Some(&1));

        fn plus_one(x: usize) -> usize {
            x + 1
        }

        assert_eq!(nest(plus_one, 0, 1), 1);
    }

    #[test]
    fn test_nest_while_list_full() {
        //nest_while_list_full()
    }
}

// fn nest_list<B, F>(f: F, n: usize)
// where
//     B: Sized,
//     F: FnMut(B) -> B,
// {
//     let a = vec![1,2];
//     a.get(index)
// }

// fn nest_while_1<B, F, T>(mut f: F, init: &B, mut test:T) -> B
// where
//     B: Sized,
//     F: FnMut(&B) -> B,
//     T: FnMut(&B) -> bool,
// {
//     let mut accum = f(init);
//     while test(&accum) {
//         accum = f(&accum);
//     }
//     accum
// }

fn nest_while_list_full<B, F, T>(mut f: F, init: B, mut test: T, m: usize, max: usize) -> Vec<B>
where
    B: Sized + Clone,
    F: FnMut(&B) -> B,
    T: FnMut(&[B]) -> bool,
{
    let mut out = vec![init];
    let mut count = 0;
    //let args = rbl_circular_buffer::CircularBuffer::<B>::new(m);

    // will always do at least m
    for _ in 1..m {
        count += 1;
        out.push(f(out.last().expect("not possible")));
    }

    let test_arg_start: usize = 0;

    while count <= max && test(&out.as_slice()[test_arg_start..test_arg_start + m - 1]) {
        count += 1;
        test_arg_start + 1;
        out.push(f(out.last().expect("not possible")));
    }

    out
}

// fn fixed_point_list_full<F, T>(f: F, t: T){
//     todo!()
// }

// fn fixed_point_list(){
//     todo!()
// }

// fn fold_list(){
//     todo!()
// }
