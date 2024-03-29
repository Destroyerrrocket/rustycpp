use std::cmp::Ordering;
use std::collections::VecDeque;

use super::MacroIntConstantExpressionAst::PreTokenIf;
use crate::Utils::Structs::{
    CompileError, CompileMsg, CompileMsgImpl, CompileWarning, FileTokPos, TokPos,
};

/// Evaluator of a macro constant expression. The standard defines a pretty low
/// lower limit in integer representation, so we use i128, which is way bigger.
type Out = Result<(i128, Vec<CompileMsg>), Vec<CompileMsg>>;
type In<'a> = (
    &'a mut VecDeque<FileTokPos<PreTokenIf>>,
    &'a FileTokPos<PreTokenIf>,
);

macro_rules! matchesP {
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
}

macro_rules! armP {
    ( $x:pat ) => {
        FileTokPos {
            tokPos: TokPos { tok: $x, .. },
            ..
        }
    };
}

pub fn exprRes(i: In) -> Out {
    let (i, s) = i;
    let (n, mut err) = expression((i, s))?;
    if !i.is_empty() {
        err.push(CompileWarning::fromPreTo(
            "the rest of the expression is not evaluated.",
            &i[0],
        ));
    }
    Ok((n, err))
}

fn literal(i: In) -> Out {
    let (i, s) = i;
    if matchesP!(i, PreTokenIf::Num(_)) {
        if let Some(armP!(PreTokenIf::Num(n))) = i.pop_front() {
            return Ok((n, vec![]));
        }
    }
    if i.is_empty() {
        Err(vec![CompileError::fromPreTo(
            "expected a number at the end of this expression:",
            s,
        )])
    } else {
        Err(vec![CompileError::fromPreTo(
            "expected a number, instead found",
            &i[0],
        )])
    }
}

fn primary_expression(i: In) -> Out {
    let (i, s) = i;
    if !matchesP!(i, PreTokenIf::LParen) {
        return literal((i, s));
    }
    let lparen = i.pop_front().unwrap();
    let (n, mut err) = expression((i, s))?;
    if matchesP!(i, PreTokenIf::RParen) {
        i.pop_front();
    } else {
        err.push(CompileError::fromPreTo(
            "expected a ')' for this '('",
            &lparen,
        ));
    }
    Ok((n, err))
}

fn expression(i: In) -> Out {
    let (i, s) = i;
    let mut n;
    let mut err = vec![];
    loop {
        let (n2, err2) = conditional_expression((i, s))?;
        n = n2;
        err.extend(err2);
        if matchesP!(i, PreTokenIf::Comma) {
            i.pop_front();
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn conditional_expression(i: In) -> Out {
    let (i, s) = i;
    let (n, mut err) = logical_or_expression((i, s))?;
    if matchesP!(i, PreTokenIf::Question) {
        i.pop_front();
        let (n2, err2) = expression((i, s))?;
        err.extend(err2);
        if matchesP!(i, PreTokenIf::Colon) {
            i.pop_front();
            let (n3, err3) = conditional_expression((i, s))?;
            err.extend(err3);
            Ok((if n == 0 { n3 } else { n2 }, err))
        } else if !i.is_empty() {
            err.push(CompileError::fromPreTo("expected a ':'", &i[0]));
            Err(err)
        } else {
            err.push(CompileError::fromPreTo("expected a ':'", s));
            Err(err)
        }
    } else {
        Ok((n, err))
    }
}

fn logical_or_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = logical_and_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::DoublePipe) {
            i.pop_front();
            let (n2, err2) = logical_and_expression((i, s))?;
            err.extend(err2);
            n = i128::from(!(n == 0 && n2 == 0));
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn logical_and_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = inclusive_or_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::DoubleAmpersand) {
            i.pop_front();
            let (n2, err2) = inclusive_or_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n != 0 && n2 != 0);
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn inclusive_or_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = exclusive_or_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Pipe) {
            i.pop_front();
            let (n2, err2) = exclusive_or_expression((i, s))?;
            err.extend(err2);
            n |= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn exclusive_or_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = and_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Caret) {
            i.pop_front();
            let (n2, err2) = and_expression((i, s))?;
            err.extend(err2);
            n ^= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn and_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = equality_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Ampersand) {
            i.pop_front();
            let (n2, err2) = equality_expression((i, s))?;
            err.extend(err2);
            n &= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn equality_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = relational_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::DoubleEqual) {
            i.pop_front();
            let (n2, err2) = relational_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n == n2);
        } else if matchesP!(i, PreTokenIf::ExclamationEqual) {
            i.pop_front();
            let (n2, err2) = relational_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n != n2);
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn relational_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = compare_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Less) {
            i.pop_front();
            let (n2, err2) = compare_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n < n2);
        } else if matchesP!(i, PreTokenIf::LessEqual) {
            i.pop_front();
            let (n2, err2) = compare_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n <= n2);
        } else if matchesP!(i, PreTokenIf::Greater) {
            i.pop_front();
            let (n2, err2) = compare_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n > n2);
        } else if matchesP!(i, PreTokenIf::GreaterEqual) {
            i.pop_front();
            let (n2, err2) = compare_expression((i, s))?;
            err.extend(err2);
            n = i128::from(n >= n2);
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn compare_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = shift_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Spaceship) {
            i.pop_front();
            let (n2, err2) = shift_expression((i, s))?;
            err.extend(err2);
            n = match n.cmp(&n2) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            };
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn shift_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = additive_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::DoubleLess) {
            i.pop_front();
            let (n2, err2) = additive_expression((i, s))?;
            err.extend(err2);
            n <<= n2;
        } else if matchesP!(i, PreTokenIf::DoubleGreater) {
            i.pop_front();
            let (n2, err2) = additive_expression((i, s))?;
            err.extend(err2);
            n >>= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn additive_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = multiplicative_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Plus) {
            i.pop_front();
            let (n2, err2) = multiplicative_expression((i, s))?;
            err.extend(err2);
            n += n2;
        } else if matchesP!(i, PreTokenIf::Minus) {
            i.pop_front();
            let (n2, err2) = multiplicative_expression((i, s))?;
            err.extend(err2);
            n -= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn multiplicative_expression(i: In) -> Out {
    let (i, s) = i;
    let (mut n, mut err) = unary_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::Star) {
            i.pop_front();
            let (n2, err2) = unary_expression((i, s))?;
            err.extend(err2);
            n *= n2;
        } else if matchesP!(i, PreTokenIf::Slash) {
            i.pop_front();
            let (n2, err2) = unary_expression((i, s))?;
            err.extend(err2);
            n /= n2;
        } else if matchesP!(i, PreTokenIf::Percent) {
            i.pop_front();
            let (n2, err2) = unary_expression((i, s))?;
            err.extend(err2);
            n %= n2;
        } else {
            break;
        }
    }
    Ok((n, err))
}

fn unary_expression(i: In) -> Out {
    let (i, s) = i;
    if matchesP!(i, PreTokenIf::Plus) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((n, err))
    } else if matchesP!(i, PreTokenIf::Minus) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((-n, err))
    } else if matchesP!(i, PreTokenIf::Exclamation) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((i128::from(n == 0), err))
    } else if matchesP!(i, PreTokenIf::Tilde) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((!n, err))
    } else if matchesP!(i, PreTokenIf::DoublePlus) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((n + 1, err))
    } else if matchesP!(i, PreTokenIf::DoubleMinus) {
        i.pop_front();
        let (n, err) = unary_expression((i, s))?;
        Ok((n - 1, err))
    } else {
        let (n, err) = postfix_expression((i, s))?;
        Ok((n, err))
    }
}

fn postfix_expression(i: In) -> Out {
    let (i, s) = i;
    let (n, mut err) = primary_expression((i, s))?;
    loop {
        if matchesP!(i, PreTokenIf::DoublePlus) {
            let t = i.pop_front().unwrap();
            err.push(CompileWarning::fromPreTo(
                "Postincrement in macro expression does nothing",
                &t,
            ));
        } else if matchesP!(i, PreTokenIf::DoubleMinus) {
            let t = i.pop_front().unwrap();
            err.push(CompileWarning::fromPreTo(
                "Postdecrement in macro expression does nothing",
                &t,
            ));
        } else {
            break;
        }
    }
    Ok((n, err))
}
