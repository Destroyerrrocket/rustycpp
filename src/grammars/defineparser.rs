use std::collections::VecDeque;

use crate::preprocessor::pretoken::PreprocessingOperator;
use crate::utils::structs::{CompileError, CompileMsgImpl, TokPos};
use crate::{p_alt, p_guard_condition_select, pvec_accumulate_while, pvoid_drop_until_fail, wrap};

use crate::{
    preprocessor::pretoken::PreToken,
    utils::structs::{CompileMsg, FileTokPos},
};

use super::defineast::{PreTokenDefine, PreTokenDefinePreParse};

type ParseRes = Result<PreTokenDefine, CompileMsg>;
type In<'a> = &'a mut VecDeque<FileTokPos<PreTokenDefinePreParse>>;

macro_rules! matchesPP {
    ( $file:expr, $x:pat ) => {
        !$file.is_empty()
            && matches!(
                $file[0],
                FileTokPos {
                    tokPos: TokPos { tok: $x, .. },
                    ..
                }
            )
    };
    ( $file:expr, $x:pat, $n:expr ) => {
        $file.len() > $n
            && matches!(
                $file[$n],
                FileTokPos {
                    tokPos: TokPos { tok: $x, .. },
                    ..
                }
            )
    };
}

macro_rules! armPP {
    ( $x:pat ) => {
        FileTokPos {
            tokPos: TokPos { tok: $x, .. },
            ..
        }
    };
}

fn normal(input: FileTokPos<PreTokenDefinePreParse>) -> PreTokenDefine {
    if let armPP!(PreTokenDefinePreParse::Normal(ref d)) = input {
        PreTokenDefine::Normal(FileTokPos::new_meta_c(d.clone(), &input))
    } else {
        panic!("Expected a normal token")
    }
}

fn ReTokWhiteSp(input: In) -> ParseRes {
    if matchesPP!(
        input,
        PreTokenDefinePreParse::Normal(PreToken::Whitespace(_))
    ) {
        let norm = normal(input.pop_front().unwrap());
        Ok(norm)
    } else {
        Err(CompileError::fromPreTo(
            "Expected a whitespace".to_string(),
            &input[0],
        ))
    }
}

fn ReTokNoWhiteSp(input: In) -> ParseRes {
    if matchesPP!(
        input,
        PreTokenDefinePreParse::Normal(PreToken::Whitespace(_))
    ) {
        Err(CompileError::fromPreTo(
            "Expected a normal token, found a whitespace".to_string(),
            &input[0],
        ))
    } else if matchesPP!(input, PreTokenDefinePreParse::Normal(_)) {
        let norm = normal(input.pop_front().unwrap());
        Ok(norm)
    } else {
        Err(CompileError::fromPreTo(
            "Expected a normal token".to_string(),
            &input[0],
        ))
    }
}

fn ArgMVar(input: In) -> ParseRes {
    if let Some(e) = input.pop_front() {
        match e {
            armPP!(PreTokenDefinePreParse::Arg(ref a)) => {
                Ok(PreTokenDefine::Arg(FileTokPos::new_meta_c(a.clone(), &e)))
            }
            armPP!(PreTokenDefinePreParse::VariadicArg) => {
                Ok(PreTokenDefine::VariadicArg(FileTokPos::new_meta_c((), &e)))
            }
            armPP!(PreTokenDefinePreParse::VariadicOpt) => {
                while ReTokWhiteSp(input).is_ok() {}
                if matchesPP!(input, PreTokenDefinePreParse::VariadicOptParenL) {
                    input.pop_front();
                    let d = parseMacroDefinition(input)?;
                    if matchesPP!(input, PreTokenDefinePreParse::VariadicOptParenR) {
                        input.pop_front();
                        return Ok(PreTokenDefine::VariadicOpt(
                            FileTokPos::new_meta_c((), &e),
                            d.into_iter().collect::<Vec<PreTokenDefine>>(),
                        ));
                    } else {
                        return Err(CompileError::fromPreTo(
                            "Expected a ) at the end of the variadic optional".to_string(),
                            &e,
                        ));
                    }
                } else {
                    return Err(CompileError::fromPreTo(
                        "Expected a ( at the start of the variadic optional".to_string(),
                        &e,
                    ));
                }
            }
            _ => {
                input.push_front(e);
                Err(CompileError::fromPreTo(
                    "Expected a parameter at:".to_string(),
                    &input[0],
                ))
            }
        }
    } else {
        Err(CompileError::fromPreTo(
            "Expected a parameter at the end of the macro definition".to_string(),
            &input[0],
        ))
    }
}

fn MaybeHashHash(input: In) -> ParseRes {
    if matchesPP!(input, PreTokenDefinePreParse::Hash)
        && matchesPP!(input, PreTokenDefinePreParse::HashHash, 1)
    {
        let hash1 = input.pop_front().unwrap();
        let hashHash = input.pop_front().unwrap();
        pvoid_drop_until_fail!(In, ReTokWhiteSp)(input);
        if matchesPP!(input, PreTokenDefinePreParse::Hash) {
            let hash2 = input.pop_front().unwrap();
            return Ok(PreTokenDefine::HashHash(
                FileTokPos::new_meta_c((), &hashHash),
                vec![PreTokenDefine::Normal(FileTokPos::new_meta_c(
                    PreToken::PreprocessingOperator(PreprocessingOperator::Hash),
                    &hash1,
                ))],
                vec![PreTokenDefine::Normal(FileTokPos::new_meta_c(
                    PreToken::PreprocessingOperator(PreprocessingOperator::Hash),
                    &hash2,
                ))],
            ));
        }
        return Err(CompileError::fromPreTo(
            "Expected a # at the end of this ##".to_string(),
            &hashHash,
        ));
    }
    if matchesPP!(input, PreTokenDefinePreParse::HashHash) {
        let unexpectedHashHash = input.pop_front().unwrap();
        return Err(CompileError::fromPreTo(
            "Expected an argument or token previous to this ##".to_string(),
            &unexpectedHashHash,
        ));
    }
    let argOrTok = p_guard_condition_select!(
        In,
        |input: In| {
            Err(CompileError::fromPreTo(
                "Expected a parameter or token".to_string(),
                &input[0],
            ))
        },
        (
            |input: In| {
                matchesPP!(
                    input,
                    PreTokenDefinePreParse::Arg(_)
                        | PreTokenDefinePreParse::VariadicArg
                        | PreTokenDefinePreParse::VariadicOpt
                )
            },
            ArgMVar
        ),
        (
            |input: In| {
                matchesPP!(input, PreTokenDefinePreParse::Normal(_))
                    && !matchesPP!(
                        input,
                        PreTokenDefinePreParse::Normal(PreToken::Whitespace(_))
                    )
            },
            ReTokNoWhiteSp
        )
    )(input)?;

    if matchesPP!(input, PreTokenDefinePreParse::HashHash) {
        let hashHash = input.pop_front().unwrap();
        pvoid_drop_until_fail!(In, ReTokWhiteSp)(input);
        let right = MaybeHashHash(input)?;
        return Ok(PreTokenDefine::HashHash(
            FileTokPos::new_meta_c((), &hashHash),
            vec![argOrTok],
            vec![right],
        ));
    } else {
        return Ok(argOrTok);
    }
}

fn NoWhiteSp(input: In) -> ParseRes {
    println!("{input:?}");
    let onHashPrecondition = wrap!(
        In,
        p_alt!(In, MaybeHashHash, |input: In| {
            let h = input.pop_front().unwrap();
            pvoid_drop_until_fail!(In, ReTokWhiteSp)(input);
            let a = MaybeHashHash(input)?;
            Ok(PreTokenDefine::Hash(
                FileTokPos::new_meta_c((), &h),
                vec![a],
            ))
        })
    );
    p_guard_condition_select!(
        In,
        MaybeHashHash,
        (
            |input: In| { matchesPP!(input, PreTokenDefinePreParse::Hash) },
            onHashPrecondition
        )
    )(input)
}

fn parseElem(input: In) -> ParseRes {
    if input.is_empty() {
        return Err(CompileError::fromPreTo(
            "Expected a token".to_string(),
            &input[0],
        ));
    }
    p_alt!(In, ReTokWhiteSp, NoWhiteSp)(input)
}

pub fn parseMacroDefinition(
    input: &mut VecDeque<FileTokPos<PreTokenDefinePreParse>>,
) -> Result<Vec<PreTokenDefine>, CompileMsg> {
    pvec_accumulate_while!(In, parseElem, |input: In| {
        !input.is_empty() && !matchesPP!(input, PreTokenDefinePreParse::VariadicOptParenR)
    })(input)
}
