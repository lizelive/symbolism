use std::collections::HashMap;

use num::Zero;

struct NestList<B: Sized, F> {
    f: F,
    state: Option<B>,
    count: usize,
    times: usize,
}

impl<B: Sized, F> NestList<B, F> {
    fn new(f: F, state: B, times: usize) -> Self { Self { f, state: Some(state), count: 0, times } }
}

struct NestWhileList<B: Sized, F> {
    f: F,
    state: Option<B>,
}

impl<B, F> Iterator for NestList<B, F>
where
    F: FnMut(B) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        let done_before = self.count;
        if done_before >= self.times {
            return None;
        }
        self.count = done_before + 1;
        if let Some(state) = self.state {
            if !done_before.is_zero() {
                self.state = (self.f)(state);
            }
            self.state
        } else {
            None
        }
    }
}

// impl<B: Sized, F: FnMut(B) -> B> NestList<B, F> {
//     fn new() -> Self {
//         let mut f = |x| x;
//         let state = f(1);
//         Self {
//             f,
//             state,
//             limit: 1,
//         }
//     }
// }

fn nest_list<B, F>(mut f: F, init: B, n: usize) -> NestList<B, F>
where
    B: Sized,
    F: FnMut(B) -> Option<B>,
{
    NestList::new(f, init, n)
}

struct TakeList{

}



struct Partition<'a, T>{
    length: usize,
    offset: usize,
    index: usize,
    list: &'a [T],
}

impl<T> Iterator for Partition<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out_size = min(self.list.len() - self.index, length);
        self.index += self.offset;
        
        self.list.array_windows()
        todo!()
    }
}
use std::hash::Hash;

fn gather_by<T, K>(list: impl IntoIterator<Item=T>, mut by: impl FnMut(T) -> K) where K: Eq + Hash {
    let dict:HashMap<K, T> = list.into_iter().map(|i|(by(i),i)).collect();
}

//fn fold_pair_list<TState, TOut, FNext>

// fn fold_list<B,F>(mut f: F, init: B, itter: impl Iterator)
// B: Sized,
// F: FnMut(B) -> Option<B>,
// {

// }

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

#[cfg(test)]
mod tests {
    use crate::nest::nest;

    #[test]
    fn it_works() {
        let v = vec![1, 2];
        assert_eq!(v.get(1), Some(&1));

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

// fn fixed_point_list_full<F, T>(f: F, t: T){
//     todo!()
// }

// fn fixed_point_list(){
//     todo!()
// }

// fn fold_list(){
//     todo!()
// }
