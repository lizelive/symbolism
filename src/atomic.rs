use crate::number::{AnyInteger, AnyRational};

enum AllAtomic {
    Head,
    AtomQ,
    //Symbols
    Symbol,
    SymbolName,
    Context,
    Names,
    NameQ,
    Remove,
    NewSymbol,
    ValueQ,
    OwnValues,
    DownValues,
    UpValues,
    Options,
    Attributes,
    Information,
    Definition,
    Save,
    FullDefinition,
    ExcludedContexts,
    IncludedContexts,
    Unique,

    ToString,
    ToExpression,

    //String
    String,
    StringQ,
    Characters,
    ToCharacterCode,
    StringLength,

    Round,
    Floor,
    Ceiling,
    IntegerPart,
    FractionalPart,
    MixedFractionParts,
    Min,
    Max,
    RealAbs,
    RealSign,
    Abs,
    Sign,
    Clip,
    Rescale,
    Chop,
    Threshold,
    LogisticSigmoid,
    Unitize,
    UnitStep,
    Ramp,
    UnitBox,
    Piecewise,
    Boole,
    DiscreteIndicator,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    NumericalOrder,
    SquareWave,
    TriangleWave,
    SawtoothWave,

    //Numbers
    Integer,
    Real,
    Rational,
    Complex,
    NumberQ,
    IntegerDigits,

    IntegerLength,
    IntegerExponent,
    BitAnd,
    BitXor,
    DigitCount,
    NumberDigit,
    Mod,

    RealDigits,
    Precision,
    MachineNumberQ,
    NumberExpand,
    FromDigits,
    NumberDecompose,
    NumberCompose,
    IntegerQ,
    ExactNumberQ,
    InexactNumberQ,

    Numerator,
    Denominator,
    NumeratorDenominator,
    Re,
    Im,
    ReIm,
    Arg,
    AbsArg,
    Conjugate,
    Quotient,
    Gcd,
    ModularInverse,

    // basic operators
    Plus,
    Minus,
    Subtract,
    
    Sub,
    Times,
}

//bfn(Numerator[x_Integer])
fn numerator_integer(x: AnyInteger) -> AnyInteger {
    todo!()
}
//bfn(Numerator[x_Rational])
fn numerator_rational(x: AnyRational) -> AnyInteger {
    todo!()
}
