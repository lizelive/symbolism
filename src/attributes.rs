pub enum Attributes {
    Constant,        //	all derivatives of f are zero
    Flat,            //	f is associative
    HoldAll,         //	all the arguments of f are not evaluated
    HoldAllComplete, //	the arguments of f are completely shielded from evaluation
    HoldFirst,       //	the first argument of f is not evaluated
    HoldRest,        //	all but the first argument of f are not evaluated
    Listable,        //	f is automatically "threaded" over lists
    Locked,          //	attributes of f cannot be changed
    NHoldAll,        //	the arguments of f are not affected by N
    NHoldFirst,      //	the first argument of f is not affected by N
    NHoldRest,       //	all but the first argument of f are not affected by N
    NumericFunction, //	the value of f is assumed to be a number when its arguments are numbers
    OneIdentity,     //	f[a], f[f[a]], etc. are equivalent to a in pattern matching
    Orderless,       //	f is commutative
    Protected,       //	values of f cannot be changed
    ReadProtected,   //	values of f cannot be read
    SequenceHold,    //	Sequence objects in the arguments of f are not flattened out
    Stub,            //	Needs is automatically called if the symbol is ever input
    Temporary,       //	f is a local variable, removed when no longer used
}
