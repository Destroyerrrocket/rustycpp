macro_rules! m {
    ($id:ident) => {
        mod $id;
        pub use $id::*;
    };
}

m! {ParseAttribute}
m! {ParseDeclaration}
m! {ParseMiscUtils}
m! {ParseNestedNameSpecifier}
m! {ParseTu}

/**
 * Used in some parse rules to indicate if it successfully matched the rule.
 */
pub enum ParseMatched {
    /// The macro was matched, and the tokens were consumed. Might return None (or equivalent) in error cases.
    Matched,
    /// The macro was not matched, and the token was not consumed.
    NotMatched,
}

impl ParseMatched {
    const fn matched(&self) -> bool {
        match self {
            Self::Matched => true,
            Self::NotMatched => false,
        }
    }
}
