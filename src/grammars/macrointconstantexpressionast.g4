

parser grammar macrointconstantexpressionast;
@tokenfactory{
pub type LocalTokenFactory<'input> = crate::utils::antlrlexerwrapper::AntlrLexerWrapperFactory<'input, crate::grammars::macrointconstantexpressionast::PreTokenIf>;
}
tokens {Num,
    LParen,
    RParen,
    Colon,
    Question,
    Tilde,
    Exclamation,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Ampersand,
    Pipe,
    DoubleEqual,
    ExclamationEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Spaceship,
    DoubleAmpersand,
    DoublePipe,
    DoubleLess,
    DoubleGreater,
    DoublePlus,
    DoubleMinus,
    Comma,
    And,
    Or,
    Xor,
    Not,
    Bitand,
    Bitor,
    Compl
}

//////////////////////////////////////////////////
/// Parser Rules
//////////////////////////////////////////////////
exprRes: expr # Result;

expr : LParen expr RParen # Paren
| expr op=(DoubleMinus|DoublePlus) # SinglePostIncrement
| <assoc=right> op=(DoubleMinus|DoublePlus) expr # SinglePreIncrement
| <assoc=right> op=(Minus|Plus) expr # UnarySign
| <assoc=right> op=(Exclamation|Not|Tilde|Compl) expr # LogicalOrBitNot
| expr op=(Star|Slash|Percent) expr # MulDivMod
| expr op=(Plus|Minus) expr # AddSub
| expr op=(DoubleLess|DoubleGreater) expr # BitShift
| expr Spaceship expr # Spaceship
| expr op=(Less|LessEqual|Greater|GreaterEqual) expr # Compare
| expr op=(DoubleEqual|ExclamationEqual) expr # Equalities
| expr (Ampersand|Bitand) expr # BitAnd
| expr (Caret|Xor) expr # BitXor
| expr (Pipe|Or) expr # BitOr
| expr (DoubleAmpersand|And) expr # LogAnd
| expr (DoublePipe|Or) expr # LogOr
| <assoc=right> expr Question expr Colon expr # Ternary
| expr Comma expr # Comma
| Num # Number
;
