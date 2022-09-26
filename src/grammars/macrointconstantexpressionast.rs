use std::fmt::Debug;

use crate::utils::pretoken::PreToken;

#[derive(Debug, Clone)]
pub enum PreTokenIf {
    Num(i128),
    Operation(PreToken),
}
