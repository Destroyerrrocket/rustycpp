use deriveMacros::CommonAst;

use crate::parse::bufferedLexer::StateBufferedLexer;
use crate::utils::structs::SourceRange;

#[derive(Clone, Copy)]
pub enum Kind {
    AlignAs,
    CXX11,
}

impl ToString for Kind {
    fn to_string(&self) -> String {
        match self {
            Self::AlignAs => "alignas".to_string(),
            Self::CXX11 => "CXX11".to_string(),
        }
    }
}

#[derive(Clone, Copy, CommonAst)]
pub struct AstAttribute {
    /// CXX11, alignas, etc.
    #[AstToString]
    kind: Kind,
    /// The range of the attribute in the source code. Includes the brackets/alignas/etc.
    pub sourceRange: SourceRange,
    /// The range of the tokens in the parser. Excludes the brackets/alignas/etc.
    parserRange: StateBufferedLexer,
}

impl AstAttribute {
    pub fn new(kind: Kind, sourceRange: SourceRange, parserRange: StateBufferedLexer) -> Self {
        Self {
            kind,
            sourceRange,
            parserRange,
        }
    }
}
