enum LeafNode {
    Integer(String),
    String(String),
    Real(String),
    Symbol(String),
}

enum CallNode{
    Leaf(LeafNode, Vec<Ast>),
    Ast(Box<Ast>, Vec<Ast>)
}

enum Ast {
    Leaf(LeafNode),
    Call(CallNode)
}

enum Node{
    LeafNode
}