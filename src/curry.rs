use std::hash::Hash;

// macro_rules! cury {
//     () => {
        
//     };
// }

trait Curry {
    
}

fn identiy<T>(x:T) -> T{
    x
}


macro_rules! curry1 {
    () =>{
        |x| x
    };
    ($f1:ident, $f2:ident) =>{
        |x| $f1($f2(x))
    };
    // ($($name:ident)+, ) => {
    //     f
    // };
}

macro_rules! composition {
    () =>{
        |x| x
    };
    ($f1:ident, $f2:ident) =>{
        |x| $f1($f2(x))
    };
    // ($($name:ident)+, ) => {
    //     f
    // };
}


// fn composition<FO,FI,FC,TI,TM,TO>(fo:FO, fi: FI)
// -> FC
// where 
//     FO: (FnMut(TM) -> TO),
//     FI: (FnMut(TI) -> TM),
//     FC: (FnMut(TI) -> TO),
// {
//     let f =composition!{fo, fi};
//     // |i| fo(fi(i));
//     // fn composition(x: TI) -> TO{
//     //     fo(fi(x))
//     // }
//     // composition
//     //todo!("nice")
// }

fn construct<Args, O: Sized, F: FnMut(Args) -> O >(f: F, args : Args) -> O{
    let g = |x:isize|1;
    g.call_once(args);
    //g.call_once(args)
    todo!()
    //f.call_mut(args)
}

fn construct<Args, O: Sized, F: FnMut(Args) -> O >(f: F, args : Args) -> O{
    let g = |x:isize,y:isize|1;
    //g.call_once(args)
    todo!()
    //f.call_mut(args)
}