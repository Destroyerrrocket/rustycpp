use std::fmt::Debug;

use crate::utils::pretoken::PreToken;

#[derive(Debug)]
pub enum IsVariadic {
	True(String),
	False,
}

impl PartialEq for IsVariadic {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
			(Self::False, Self::False) |
            (Self::True(_), Self::True(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum DefineAst {
	Define(String, Option<Vec<String>>, IsVariadic, Vec<PreToken>),
	Err,
}
