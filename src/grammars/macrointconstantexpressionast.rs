//! Evaluator of the macro integer constant expression ast.
use std::fmt::Debug;

use crate::grammars::generated::macrointconstantexpressionast::{
    macrointconstantexpressionastContextType, macrointconstantexpressionastVisitorCompat,
    AddSubContext, AddSubContextAttrs, BitAndContext, BitAndContextAttrs, BitOrContext,
    BitOrContextAttrs, BitShiftContext, BitShiftContextAttrs, BitXorContext, BitXorContextAttrs,
    CommaContext, CommaContextAttrs, CompareContext, CompareContextAttrs, EqualitiesContext,
    EqualitiesContextAttrs, ExprResContextAll, LogAndContext, LogAndContextAttrs, LogOrContext,
    LogOrContextAttrs, LogicalOrBitNotContext, LogicalOrBitNotContextAttrs, MulDivModContext,
    MulDivModContextAttrs, NumberContext, NumberContextAttrs, ParenContext, ParenContextAttrs,
    ResultContext, ResultContextAttrs, SinglePostIncrementContext, SinglePostIncrementContextAttrs,
    SinglePreIncrementContext, SinglePreIncrementContextAttrs, SpaceshipContext,
    SpaceshipContextAttrs, TernaryContext, TernaryContextAttrs, UnarySignContext,
    UnarySignContextAttrs,
};
use crate::utils::antlrlexerwrapper::HasEOF;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;

#[derive(Debug, Clone)]
#[repr(isize)]
#[doc(hidden)]
pub enum PreTokenIf {
    EOF = -1,
    Invalid = 0,
    Num(i128) = 1,
    LParen = 2,
    RParen = 3,
    Colon = 4,
    Question = 5,
    Tilde = 6,
    Exclamation = 7,
    Plus = 8,
    Minus = 9,
    Star = 10,
    Slash = 11,
    Percent = 12,
    Caret = 13,
    Ampersand = 14,
    Pipe = 15,
    DoubleEqual = 16,
    ExclamationEqual = 17,
    Less = 18,
    Greater = 19,
    LessEqual = 20,
    GreaterEqual = 21,
    Spaceship = 22,
    DoubleAmpersand = 23,
    DoublePipe = 24,
    DoubleLess = 25,
    DoubleGreater = 26,
    DoublePlus = 27,
    DoubleMinus = 28,
    Comma = 29,
    And = 30,
    Or = 31,
    Xor = 32,
    Not = 33,
    Bitand = 34,
    Bitor = 35,
    Compl = 36,
    AndEq = 37,
    OrEq = 38,
    XorEq = 39,
    NotEq = 40,
}

impl std::fmt::Display for PreTokenIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreTokenIf::EOF => write!(f, "EOF"),
            PreTokenIf::Invalid => write!(f, "Invalid"),
            PreTokenIf::Num(n) => write!(f, "{}", n),
            PreTokenIf::LParen => write!(f, "("),
            PreTokenIf::RParen => write!(f, ")"),
            PreTokenIf::Colon => write!(f, ":"),
            PreTokenIf::Question => write!(f, "?"),
            PreTokenIf::Tilde => write!(f, "~"),
            PreTokenIf::Exclamation => write!(f, "!"),
            PreTokenIf::Plus => write!(f, "+"),
            PreTokenIf::Minus => write!(f, "-"),
            PreTokenIf::Star => write!(f, "*"),
            PreTokenIf::Slash => write!(f, "/"),
            PreTokenIf::Percent => write!(f, "%"),
            PreTokenIf::Caret => write!(f, "^"),
            PreTokenIf::Ampersand => write!(f, "&"),
            PreTokenIf::Pipe => write!(f, "|"),
            PreTokenIf::DoubleEqual => write!(f, "=="),
            PreTokenIf::ExclamationEqual => write!(f, "!="),
            PreTokenIf::Less => write!(f, "<"),
            PreTokenIf::Greater => write!(f, ">"),
            PreTokenIf::LessEqual => write!(f, "<="),
            PreTokenIf::GreaterEqual => write!(f, ">="),
            PreTokenIf::Spaceship => write!(f, "<=>"),
            PreTokenIf::DoubleAmpersand => write!(f, "&&"),
            PreTokenIf::DoublePipe => write!(f, "||"),
            PreTokenIf::DoubleLess => write!(f, "<<"),
            PreTokenIf::DoubleGreater => write!(f, ">>"),
            PreTokenIf::DoublePlus => write!(f, "++"),
            PreTokenIf::DoubleMinus => write!(f, "--"),
            PreTokenIf::Comma => write!(f, ","),
            PreTokenIf::And => write!(f, "and"),
            PreTokenIf::Or => write!(f, "or"),
            PreTokenIf::Xor => write!(f, "xor"),
            PreTokenIf::Not => write!(f, "not"),
            PreTokenIf::Bitand => write!(f, "bitand"),
            PreTokenIf::Bitor => write!(f, "bitor"),
            PreTokenIf::Compl => write!(f, "compl"),
            PreTokenIf::AndEq => write!(f, "and_eq"),
            PreTokenIf::OrEq => write!(f, "or_eq"),
            PreTokenIf::XorEq => write!(f, "xor_eq"),
            PreTokenIf::NotEq => write!(f, "not_eq"),
        }
    }
}

impl PreTokenIf {
    #[doc(hidden)]
    pub fn stringToPreTokenIfOperator(s: &str) -> PreTokenIf {
        match s {
            r"(" => PreTokenIf::LParen,
            r")" => PreTokenIf::RParen,
            r":" => PreTokenIf::Colon,
            r"?" => PreTokenIf::Question,
            r"~" => PreTokenIf::Tilde,
            r"!" => PreTokenIf::Exclamation,
            r"+" => PreTokenIf::Plus,
            r"-" => PreTokenIf::Minus,
            r"*" => PreTokenIf::Star,
            r"/" => PreTokenIf::Slash,
            r"%" => PreTokenIf::Percent,
            r"^" => PreTokenIf::Caret,
            r"&" => PreTokenIf::Ampersand,
            r"|" => PreTokenIf::Pipe,
            r"==" => PreTokenIf::DoubleEqual,
            r"!=" => PreTokenIf::ExclamationEqual,
            r"<" => PreTokenIf::Less,
            r">" => PreTokenIf::Greater,
            r"<=" => PreTokenIf::LessEqual,
            r">=" => PreTokenIf::GreaterEqual,
            r"<=>" => PreTokenIf::Spaceship,
            r"&&" => PreTokenIf::DoubleAmpersand,
            r"||" => PreTokenIf::DoublePipe,
            r"<<" => PreTokenIf::DoubleLess,
            r">>" => PreTokenIf::DoubleGreater,
            r"++" => PreTokenIf::DoublePlus,
            r"--" => PreTokenIf::DoubleMinus,
            r"," => PreTokenIf::Comma,
            r"and" => PreTokenIf::And,
            r"or" => PreTokenIf::Or,
            r"xor" => PreTokenIf::Xor,
            r"not" => PreTokenIf::Not,
            r"bitand" => PreTokenIf::Bitand,
            r"bitor" => PreTokenIf::Bitor,
            r"compl" => PreTokenIf::Compl,
            _ => unreachable!(),
        }
    }
}

impl HasEOF for PreTokenIf {
    fn getEOF() -> Self {
        PreTokenIf::EOF
    }
    fn getInvalid() -> Self {
        PreTokenIf::Invalid
    }

    fn getFromTType(ttype: isize) -> Self {
        match ttype {
            -1 => Self::EOF,
            0 => Self::Invalid,
            1 => Self::Num(0),
            2 => Self::LParen,
            3 => Self::RParen,
            4 => Self::Colon,
            5 => Self::Question,
            6 => Self::Tilde,
            7 => Self::Exclamation,
            8 => Self::Plus,
            9 => Self::Minus,
            10 => Self::Star,
            11 => Self::Slash,
            12 => Self::Percent,
            13 => Self::Caret,
            14 => Self::Ampersand,
            15 => Self::Pipe,
            16 => Self::DoubleEqual,
            17 => Self::ExclamationEqual,
            18 => Self::Less,
            19 => Self::Greater,
            20 => Self::LessEqual,
            21 => Self::GreaterEqual,
            22 => Self::Spaceship,
            23 => Self::DoubleAmpersand,
            24 => Self::DoublePipe,
            25 => Self::DoubleLess,
            26 => Self::DoubleGreater,
            27 => Self::DoublePlus,
            28 => Self::DoubleMinus,
            29 => Self::Comma,
            30 => Self::And,
            31 => Self::Or,
            32 => Self::Xor,
            33 => Self::Not,
            34 => Self::Bitand,
            35 => Self::Bitor,
            36 => Self::Compl,
            37 => Self::AndEq,
            38 => Self::OrEq,
            39 => Self::XorEq,
            40 => Self::NotEq,
            _ => Self::Invalid,
        }
    }
}

/// Evaluator of a macro constant expression. The standard defines a pretty low
/// lower limit in integer representation, so we use i128, which is way bigger.
pub struct VisitorEvaluator(pub i128, i128);

impl<'a> VisitorEvaluator {
    #[doc(hidden)]
    pub fn new() -> Self {
        Self(0, 0)
    }

    /// Result of the evaluation.
    pub fn res(&self) -> i128 {
        self.0
    }

    /// Sart evaluation.
    pub fn visit_start(&mut self, ctx: &ExprResContextAll<'a>) {
        self.visit(ctx);
    }
}

impl<'a> ParseTreeVisitorCompat<'a> for VisitorEvaluator {
    type Node = macrointconstantexpressionastContextType;

    type Return = i128;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.1
    }
}

impl<'input> macrointconstantexpressionastVisitorCompat<'input> for VisitorEvaluator {
    fn visit_Number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
        if let PreTokenIf::Num(n) = ctx.Num().unwrap().symbol.get_text().data.tokPos.tok {
            return n;
        }
        unreachable!()
    }

    fn visit_Result(&mut self, ctx: &ResultContext<'input>) -> Self::Return {
        let res = self.visit(&*ctx.expr().unwrap());
        log::info!("evaluated to {}", res);
        self.0 = res;
        return res;
    }

    fn visit_SinglePostIncrement(
        &mut self,
        ctx: &SinglePostIncrementContext<'input>,
    ) -> Self::Return {
        // TODO Warn of useless increment?
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_UnarySign(&mut self, ctx: &UnarySignContext<'input>) -> Self::Return {
        let e = self.visit(&*ctx.expr().unwrap());
        return if ctx.Minus().is_some() { -e } else { e };
    }

    fn visit_AddSub(&mut self, ctx: &AddSubContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if ctx.Plus().is_some() {
            return e0 + e1;
        } else if ctx.Minus().is_some() {
            return e0 - e1;
        } else {
            unreachable!()
        }
    }

    fn visit_BitShift(&mut self, ctx: &BitShiftContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if ctx.DoubleLess().is_some() {
            return e0 << e1;
        } else if ctx.DoubleGreater().is_some() {
            return e0 >> e1;
        } else {
            unreachable!()
        }
    }

    fn visit_Ternary(&mut self, ctx: &TernaryContext<'input>) -> Self::Return {
        let e = self.visit(&*ctx.expr(0).unwrap());
        self.visit(&*ctx.expr(if e == 0 { 2 } else { 1 }).unwrap())
    }

    fn visit_Spaceship(&mut self, ctx: &SpaceshipContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if e0 < e1 {
            return -1;
        } else if e0 > e1 {
            return 1;
        } else {
            return 0;
        }
    }

    fn visit_SinglePreIncrement(
        &mut self,
        ctx: &SinglePreIncrementContext<'input>,
    ) -> Self::Return {
        self.visit(&*ctx.expr().unwrap()) + if ctx.DoubleMinus().is_some() { -1 } else { 1 }
    }

    fn visit_LogAnd(&mut self, ctx: &LogAndContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if (e0 & e1) == 0 {
            0
        } else {
            1
        }
    }

    fn visit_Comma(&mut self, ctx: &CommaContext<'input>) -> Self::Return {
        self.visit(&*ctx.expr(1).unwrap())
    }

    fn visit_MulDivMod(&mut self, ctx: &MulDivModContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if ctx.Star().is_some() {
            return e0 * e1;
        } else if ctx.Slash().is_some() {
            return e0 / e1;
        } else if ctx.Percent().is_some() {
            return e0 % e1;
        } else {
            unreachable!()
        }
    }

    fn visit_LogicalOrBitNot(&mut self, ctx: &LogicalOrBitNotContext<'input>) -> Self::Return {
        let e = self.visit(&*ctx.expr().unwrap());
        if ctx.Exclamation().is_some() && ctx.Not().is_some() {
            return if e == 0 { 1 } else { 0 };
        } else if ctx.Tilde().is_some() && ctx.Compl().is_some() {
            return !e;
        } else {
            unreachable!()
        }
    }

    fn visit_Equalities(&mut self, ctx: &EqualitiesContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if ctx.DoubleEqual().is_some() {
            return if e0 == e1 { 1 } else { 0 };
        } else if ctx.ExclamationEqual().is_some() {
            return if e0 != e1 { 1 } else { 0 };
        } else {
            unreachable!()
        }
    }

    fn visit_BitAnd(&mut self, ctx: &BitAndContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        return e0 & e1;
    }

    fn visit_Compare(&mut self, ctx: &CompareContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        if ctx.Greater().is_some() {
            return if e0 > e1 { 1 } else { 0 };
        } else if ctx.GreaterEqual().is_some() {
            return if e0 >= e1 { 1 } else { 0 };
        } else if ctx.Less().is_some() {
            return if e0 < e1 { 1 } else { 0 };
        } else if ctx.LessEqual().is_some() {
            return if e0 <= e1 { 1 } else { 0 };
        } else {
            unreachable!()
        }
    }

    fn visit_Paren(&mut self, ctx: &ParenContext<'input>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_BitOr(&mut self, ctx: &BitOrContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        return e0 | e1;
    }

    fn visit_LogOr(&mut self, ctx: &LogOrContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );

        if (e0 | e1) == 0 {
            0
        } else {
            1
        }
    }

    fn visit_BitXor(&mut self, ctx: &BitXorContext<'input>) -> Self::Return {
        let (e0, e1) = (
            self.visit(&*ctx.expr(0).unwrap()),
            self.visit(&*ctx.expr(1).unwrap()),
        );
        return e0 ^ e1;
    }
}
