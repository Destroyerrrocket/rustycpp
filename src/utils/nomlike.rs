/// This macro is used to create a parser that will return the result of the first parser that succeeds.
#[macro_export]
macro_rules! p_alt {
    (@ $input:ident, $r:expr, $parser:expr) => {
        $r.or_else(|_| $parser($input))
    };
    (@ $input:ident, $r:expr, $parser:expr, $($parsers:expr)+) => {
        p_alt!(@ $input, $r.or_else(|_| $parser(&mut *$input)), $($parsers)+)
    };

    ($type:ty, $parser:expr) => {
        |input:$type| {
            $parser(input)
        }
    };

    ($type:ty, $parser:expr, $($parsers:expr),+) => {
        |input:$type| {
            p_alt!(@ input, $parser(&mut *input), $($parsers)+)
        }
    };
}

/// This macro is used to select a parser based on a condition.
#[macro_export]
macro_rules! p_guard_condition_select {
    (@ $error:expr, $input:ident, ($parser_cond:expr, $parser:expr)) => {
        if $parser_cond(&mut *$input) {
            return $parser($input);
        } else {return $error($input)}
    };
    (@ $error:expr, $input:ident, ($parser_cond:expr, $parser:expr), $(($parsers_cond:expr, $parsers:expr)),+) => {
        if $parser_cond(&mut *$input) {
            return $parser($input);
        } else {
            p_guard_condition_select!(@ $error, $input, $(($parsers_cond, $parsers))+)
        }
    };

    ($type:ty, $error:expr, ($parser_cond:expr, $parser:expr)) => {
        |input: $type| {
            if $parser_cond(&mut *input) {
                return $parser(input);
            } else {return $error(input);}
        }
    };

    ($type:ty, $error:expr, ($parser_cond:expr, $parser:expr), $(($parsers_cond:expr, $parsers:expr)),+) => {
        |input: $type| {
            if $parser_cond(&mut *input) {
                return $parser(input);
            } else {
                p_guard_condition_select!(@ $error, input, $(($parsers_cond, $parsers))+)
            }
        }
    };
}

/// This macro is used to accumulate all the elements of a parser until it fails or a condition is not met.
#[macro_export]
macro_rules! pvec_accumulate_while {
    ($parser:expr) => {
        let res = vec![];
        |input| loop {
            let r = $parser(input);
            if r.is_err() {
                return Ok(res);
            }
            res.push(r.unwrap());
        }
    };

    ($type:ty, $parser:expr, $eval:expr) => {
        |input: $type| {
            let mut res = vec![];
            while $eval(input) {
                let r = $parser(input);
                if r.is_err() {
                    return Ok(res);
                }
                res.push(r.unwrap());
            }
            return Ok(res);
        }
    };
}

/// This macro is used to drop all the elements of a parser until it fails.
#[macro_export]
macro_rules! pvoid_drop_until_fail {
    ($type:ty, $parser:expr) => {
        |input: $type| while $parser(&mut *input).is_ok() {}
    };
}

/// This macro is used to concatenate parsers together. If any of the parsers fail, the macro will return the result including the first parser that failed.
#[macro_export]
macro_rules! p_concatenate_return_on_fail {
    (@ $input:ident $res:ident $parser:expr) => {
        $res.push($parser($input));
        if $res[$res.len() - 1].is_err() {
            return $res;
        }
    };

    (@ $input:ident $res:ident $parser:expr, $($parsers:expr),+) => {
        $res.push($parser($input));
        if $res[$res.len() - 1].is_err() {
            return $res;
        }
        concatenate_return_on_fail!(@ $input $res $($parsers),+)
    };

    ($parser:expr) => {
        |input| vec![$parser(input)]
    };
    ($parser:expr, $($parsers:expr),+) => {
        |input| {
            let mut res = vec![$parser(input)];
            if res[0].is_err() {
                return res;
            }
        }
    };
}

/// This macro is used to wrap a parser in a closure that takes the input as a parameter. Fixes some issues with the borrow checker.
#[macro_export]
macro_rules! wrap {
    ($type:ty, $parser:expr) => {
        |input: $type| $parser(input)
    };
}
