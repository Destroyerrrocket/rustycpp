//! Wrapper over a token collection for antlr consumption.
#![allow(clippy::unused_self, clippy::cast_sign_loss, clippy::borrowed_box)]
use antlr_rust::atn_simulator::IATNSimulator;
use antlr_rust::char_stream::InputData;
use antlr_rust::errors::{ANTLRError, FailedPredicateError, InputMisMatchError, NoViableAltError};
use antlr_rust::interval_set::IntervalSet;
use antlr_rust::parser::ParserNodeType;
use antlr_rust::rule_context::{CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF, TOKEN_EPSILON, TOKEN_INVALID_TYPE};
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::transition::RuleTransition;
use antlr_rust::{ErrorStrategy, Parser};
use antlr_rust::{TidAble, TokenSource};

use std::borrow::Borrow;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::marker::PhantomData;
use std::ops::{Index, Range, RangeFrom};
use std::rc::Rc;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::{Arc, Mutex};

use super::structs::{CompileError, CompileFile, CompileMsg, FileTokPos};

#[doc(hidden)]
const ATNSTATE_INVALID_TYPE: isize = 0;
#[doc(hidden)]
const ATNSTATE_BASIC: isize = 1;
#[doc(hidden)]
const ATNSTATE_RULE_START: isize = 2;
#[doc(hidden)]
const ATNSTATE_BLOCK_START: isize = 3;
#[doc(hidden)]
const ATNSTATE_PLUS_BLOCK_START: isize = 4;
#[doc(hidden)]
const ATNSTATE_STAR_BLOCK_START: isize = 5;
#[doc(hidden)]
const ATNSTATE_TOKEN_START: isize = 6;
#[doc(hidden)]
const ATNSTATE_RULE_STOP: isize = 7;
#[doc(hidden)]
const ATNSTATE_BLOCK_END: isize = 8;
#[doc(hidden)]
const ATNSTATE_STAR_LOOP_BACK: isize = 9;
#[doc(hidden)]
const ATNSTATE_STAR_LOOP_ENTRY: isize = 10;
#[doc(hidden)]
const ATNSTATE_PLUS_LOOP_BACK: isize = 11;
#[doc(hidden)]
const ATNSTATE_LOOP_END: isize = 12;
#[doc(hidden)]
const ATNSTATE_INVALID_STATE_NUMBER: isize = -1;

/// A token type that goes into antlr must have EOF and Invalid variants
pub trait HasEOF {
    /// The EOF variant
    fn getEOF() -> Self;
    /// The Invalid variant
    fn getInvalid() -> Self;
    /// Go from a token type index to a default token of that type
    fn getFromTType(ttype: isize) -> Self;
}

#[derive(Debug, Clone)]
/// Wrapper over a token, so we also store its index
pub struct AntlrToken<T: Clone + Debug + HasEOF + Display> {
    /// The token
    pub data: FileTokPos<T>,
    /// Error in case it is found
    /// The index of the token
    index: Arc<AtomicIsize>,
}

impl<T: Clone + Debug + HasEOF + Display> AntlrToken<T> {
    /// new token with index
    pub fn new(data: FileTokPos<T>, index: isize) -> Self {
        Self {
            data,
            index: Arc::new(AtomicIsize::new(index)),
        }
    }
}

impl<T: Clone + Debug + HasEOF + Display> std::fmt::Display for AntlrToken<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.data.tokPos.tok, f)
    }
}

impl<T: Clone + Debug + HasEOF + Display + 'static> Token for AntlrToken<T> {
    type Data = Self;

    fn get_channel(&self) -> isize {
        antlr_rust::token::TOKEN_DEFAULT_CHANNEL
    }

    fn get_start(&self) -> isize {
        isize::try_from(self.data.tokPos.start).unwrap()
    }

    fn get_stop(&self) -> isize {
        isize::try_from(self.data.tokPos.end).unwrap()
    }

    fn get_line(&self) -> isize {
        isize::try_from(self.data.file.getRowColumn(self.data.tokPos.start).0).unwrap()
    }

    fn get_column(&self) -> isize {
        isize::try_from(self.data.file.getRowColumn(self.data.tokPos.start).1).unwrap()
    }

    fn set_text(&mut self, _: <Self::Data as ToOwned>::Owned) {
        unreachable!();
        //*self = tokie;
    }

    fn get_token_index(&self) -> isize {
        self.index.load(Ordering::Relaxed)
    }

    fn set_token_index(&self, v: isize) {
        self.index.store(v, Ordering::Relaxed);
    }

    fn to_owned(&self) -> OwningToken {
        OwningToken {
            token_type: self.get_token_type(),
            channel: self.get_channel(),
            start: self.get_start(),
            stop: self.get_stop(),
            token_index: AtomicIsize::new(self.get_token_index()),
            line: self.get_line(),
            column: self.get_column(),
            text: format!("{:?}", self.data.tokPos.tok),
            read_only: false,
        }
    }

    fn get_token_type(&self) -> isize {
        unsafe { *std::ptr::addr_of!(self.data.tokPos.tok).cast::<isize>() }
    }

    fn get_text(&self) -> &Self::Data {
        self
    }
}

impl<T: Clone + Debug + HasEOF + Display + 'static> InputData for AntlrToken<T> {
    // fn to_indexed_vec(&self) -> Vec<(u32, u32)>;

    #[doc(hidden)]
    fn offset(&self, _: isize, _: isize) -> Option<isize> {
        None
    }

    #[doc(hidden)]
    fn item(&self, _: isize) -> Option<isize> {
        unimplemented!();
    }

    #[doc(hidden)]
    fn len(&self) -> usize {
        /*if self.0.err.is_some() {
            return self.0.err.as_ref().unwrap().tokPos.end
                - self.err.as_ref().unwrap().tokPos.start;
        } else {
            return self.data.as_ref().unwrap().tokPos.end
                - self.data.as_ref().unwrap().tokPos.start;
        }*/
        unimplemented!();
    }

    #[doc(hidden)]
    fn from_text(_: &str) -> Self::Owned {
        /*Self::new_err(
            FileTokPos::new_meta((message.to_owned(), TOKEN_INVALID_TYPE)),
            -1,
        );*/
        unimplemented!();
    }

    #[doc(hidden)]
    fn to_display(&self) -> String {
        return format!("{}", self.data.tokPos.tok);
    }
}

impl<T: Clone + Debug + HasEOF + Display + 'static> Index<Range<usize>> for AntlrToken<T> {
    type Output = Self;

    fn index(&self, _: Range<usize>) -> &Self::Output {
        unimplemented!();
    }
}

impl<T: Clone + Debug + HasEOF + Display + 'static> Index<RangeFrom<usize>> for AntlrToken<T> {
    type Output = Self;

    fn index(&self, _: RangeFrom<usize>) -> &Self::Output {
        unimplemented!();
    }
}

#[derive(Debug)]
/// A fake factory of tokens. Never actually used in our code, but it is refered
/// in antlr code generated so it knows the token type, and `create_invalid` sometimes is called.
pub struct AntlrLexerWrapperFactory<'a, U: Clone + Debug + HasEOF + Display> {
    /// Fake a lifetime need.
    _phantom: &'a PhantomData<U>,
}

impl<'a, U: Clone + Debug + HasEOF + Display + 'static> TokenFactory<'a>
    for AntlrLexerWrapperFactory<'a, U>
{
    type Inner = AntlrToken<U>;

    type Tok = Box<Self::Inner>;

    type Data = AntlrToken<U>;

    type From = AntlrToken<U>;

    fn create<T>(
        &self,
        _: Option<&mut T>,
        _: isize,
        _: Option<<Self::Data as ToOwned>::Owned>,
        _: isize,
        _: isize,
        _: isize,
        _: isize,
        _: isize,
    ) -> Self::Tok
    where
        T: antlr_rust::char_stream::CharStream<Self::From> + ?Sized,
    {
        unimplemented!();
        /*
        Box::new(AntlrToken::new_err(
            FileTokPos::new_meta_c((text.unwrap().1, ttype), &text.unwrap().0.data.unwrap()),
            -1,
        ))*/
    }

    fn create_invalid() -> Self::Tok {
        Box::new(AntlrToken::new(FileTokPos::new_meta(U::getInvalid()), -1))
    }

    fn get_data(data: Self::From) -> std::borrow::Cow<'static, Self::Data> {
        std::borrow::Cow::Owned(data)
    }
}

unsafe impl<'a, T: Clone + Debug + HasEOF + Display + 'static> TidAble<'a>
    for AntlrLexerWrapperFactory<'a, T>
{
    type Static = ();
}

#[derive(Debug)]
/// A wrapper over a token list, intended for use as input to antlr.
pub struct AntlrLexerWrapper<'a, T: Clone + Debug + HasEOF + Display> {
    /// Fake a lifetime need.
    pd: &'a PhantomData<AntlrToken<T>>,
    /// The tokens to input.
    tokens: VecDeque<FileTokPos<T>>,
    /// Index of the current token.
    idx: isize,
    /// File name of the soure of tokens.
    file: String,
}

impl<'a, T: Clone + Debug + HasEOF + Display> AntlrLexerWrapper<'a, T> {
    /// Create a new wrapper.
    pub const fn new(tokens: VecDeque<FileTokPos<T>>, file: String) -> Self {
        Self {
            pd: &PhantomData,
            tokens,
            idx: 0,
            file,
        }
    }
}

impl<'a, T: Clone + Debug + HasEOF + Display + 'static> TokenSource<'a>
    for AntlrLexerWrapper<'a, T>
{
    type TF = AntlrLexerWrapperFactory<'a, T>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'a>>::Tok {
        if let Some(tok) = self.tokens.pop_front() {
            self.idx += 1;
            Box::new(AntlrToken::new(tok, self.idx))
        } else {
            Box::new(AntlrToken::new(FileTokPos::new_meta(T::getEOF()), self.idx))
        }
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        unimplemented!()
    }

    fn get_source_name(&self) -> String {
        self.file.clone()
    }

    fn get_token_factory(&self) -> &'a Self::TF {
        &AntlrLexerWrapperFactory {
            _phantom: &PhantomData,
        }
    }
}

unsafe impl<'a, T: Clone + Debug + Display + HasEOF + 'static> TidAble<'a>
    for AntlrLexerWrapper<'a, T>
{
    type Static = ();
}

#[derive(Debug)]
/// A wrapper over a token list, intended for use as input to antlr.
pub struct AntlrLexerIteratorWrapper<
    'a,
    T: Clone + Debug + HasEOF + Display,
    Iter: Iterator<Item = FileTokPos<T>>,
> {
    /// The tokens to input.
    tokens: &'a mut Iter,
    /// Index of the current token.
    idx: isize,
    /// File name of the soure of tokens.
    file: String,
}

impl<'a, T: Clone + Debug + Display + HasEOF, Iter: Iterator<Item = FileTokPos<T>>>
    AntlrLexerIteratorWrapper<'a, T, Iter>
{
    /// Create a new wrapper.
    pub fn new(tokens: &'a mut Iter, file: String) -> Self {
        Self {
            tokens,
            idx: 0,
            file,
        }
    }
}

impl<'a, T: Clone + Debug + HasEOF + Display + 'static, Iter: Iterator<Item = FileTokPos<T>>>
    TokenSource<'a> for AntlrLexerIteratorWrapper<'a, T, Iter>
{
    type TF = AntlrLexerWrapperFactory<'a, T>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'a>>::Tok {
        if let Some(tok) = self.tokens.next() {
            self.idx += 1;
            Box::new(AntlrToken::new(tok, self.idx))
        } else {
            Box::new(AntlrToken::new(FileTokPos::new_meta(T::getEOF()), self.idx))
        }
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        unimplemented!()
    }

    fn get_source_name(&self) -> String {
        self.file.clone()
    }

    fn get_token_factory(&self) -> &'a Self::TF {
        &AntlrLexerWrapperFactory {
            _phantom: &PhantomData,
        }
    }
}

unsafe impl<'a, T: Clone + Debug + HasEOF + Display + 'static, Iter: Iterator<Item = FileTokPos<T>>>
    TidAble<'a> for AntlrLexerIteratorWrapper<'a, T, Iter>
{
    type Static = ();
}

#[derive(Debug)]
#[doc(hidden)]
pub struct LexerWrapperErrorStrategy<'input, Ctx, Tok, Recognizer>
where
    Ctx: ParserNodeType<'input>,
    Tok: Clone + Debug + HasEOF + Display + 'input,
    Recognizer: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
{
    error_recovery_mode: bool,
    last_error_index: isize,
    last_error_states: Option<IntervalSet>,
    next_tokens_state: isize,
    next_tokens_ctx: Option<Rc<Ctx::Type>>,
    errorList: Rc<Mutex<Vec<CompileMsg>>>,
    file: Arc<CompileFile>,
    PhantomData1: PhantomData<Tok>,
    PhantomData2: PhantomData<Recognizer>,
}

unsafe impl<'input, Ctx, Tok, Recognizer> TidAble<'input>
    for LexerWrapperErrorStrategy<'input, Ctx, Tok, Recognizer>
where
    Ctx: ParserNodeType<'input>,
    Tok: Clone + Debug + HasEOF + Display,
    Recognizer: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>> + 'input,
{
    type Static = ();
}

#[doc(hidden)]
impl<'input, Ctx, Tok, Recognizer> LexerWrapperErrorStrategy<'input, Ctx, Tok, Recognizer>
where
    Ctx: ParserNodeType<'input>,
    Tok: Clone + Debug + HasEOF + Display + 'static,
    Recognizer: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>> + 'input,
{
    fn escape_whitespaces(data: impl Borrow<str>, escape_spaces: bool) -> String {
        let data = data.borrow();
        let mut res = String::with_capacity(data.len());
        data.chars().for_each(|ch| match ch {
            ' ' if escape_spaces => res.push('\u{00B7}'),
            '\t' => res.push_str("\\t"),
            '\n' => res.push_str("\\n"),
            '\r' => res.push_str("\\r"),
            _ => res.push(ch),
        });
        res
    }

    /// Creates new instance of `LexerWrapperErrorStrategy`
    pub const fn new(errorList: Rc<Mutex<Vec<CompileMsg>>>, file: Arc<CompileFile>) -> Self {
        Self {
            error_recovery_mode: false,
            last_error_index: -1,
            last_error_states: None,
            next_tokens_state: ATNSTATE_INVALID_STATE_NUMBER,
            next_tokens_ctx: None,
            errorList,
            file,
            PhantomData1: PhantomData,
            PhantomData2: PhantomData,
        }
    }

    fn begin_error_condition(&mut self, _recognizer: &Recognizer) {
        self.error_recovery_mode = true;
    }

    fn end_error_condition(&mut self, _recognizer: &Recognizer) {
        self.error_recovery_mode = false;
        self.last_error_index = -1;
        self.last_error_states = None;
    }

    fn report_no_viable_alternative<
        T: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
    >(
        &self,
        recognizer: &mut T,
        e: &NoViableAltError,
    ) -> String {
        let input = if e.start_token.token_type == TOKEN_EOF {
            "<EOF>".to_owned()
        } else {
            recognizer.get_input_stream_mut().get_text_from_interval(
                e.start_token.get_token_index(),
                e.base.offending_token.get_token_index(),
            )
        };

        format!("no viable alternative at input '{input}'")
    }

    fn report_input_mismatch<
        T: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
    >(
        &self,
        recognizer: &T,
        e: &InputMisMatchError,
    ) -> String {
        format!(
            "mismatched input {} expecting {}",
            self.get_token_error_display(&e.base.offending_token),
            e.base
                .get_expected_tokens(recognizer)
                .to_token_string(recognizer.get_vocabulary())
        )
    }

    fn report_failed_predicate(&self, recognizer: &Recognizer, e: &FailedPredicateError) -> String {
        format!(
            "rule {} {}",
            recognizer.get_rule_names()[recognizer.get_parser_rule_context().get_rule_index()],
            e.base.message
        )
    }

    fn report_unwanted_token(&mut self, recognizer: &mut Recognizer) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = &**recognizer.get_current_token();
        let token_name = self.get_token_error_display(t);
        let msg = format!("extraneous input {token_name} expecting {expecting}");
        self.errorList
            .lock()
            .unwrap()
            .push(CompileError::from_preTo(msg, &t.data));
    }

    fn report_missing_token(&mut self, recognizer: &mut Recognizer) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = &**recognizer.get_current_token();
        let _token_name = self.get_token_error_display(t);
        let msg = format!(
            "missing {} at {}",
            expecting,
            self.get_token_error_display(t)
        );
        self.errorList
            .lock()
            .unwrap()
            .push(CompileError::from_preTo(msg, &t.data));
    }

    fn single_token_insertion(&mut self, recognizer: &mut Recognizer) -> bool {
        let current_token = recognizer.get_input_stream_mut().la(1);

        let atn = recognizer.get_interpreter().atn();
        let current_state = atn.states[recognizer.get_state() as usize].as_ref();
        let next = current_state
            .get_transitions()
            .first()
            .unwrap()
            .get_target();
        let expect_at_ll2 = atn.next_tokens_in_ctx::<Ctx>(
            atn.states[next].as_ref(),
            Some(&**recognizer.get_parser_rule_context()),
        );
        if expect_at_ll2.contains(current_token) {
            self.report_missing_token(recognizer);
            return true;
        }
        false
    }

    fn single_token_deletion<'a>(
        &mut self,
        recognizer: &'a mut Recognizer,
    ) -> Option<&'a Box<AntlrToken<Tok>>> {
        let next_token_type = recognizer.get_input_stream_mut().la(2);
        let expecting = self.get_expected_tokens(recognizer);
        if expecting.contains(next_token_type) {
            self.report_unwanted_token(recognizer);
            recognizer.consume(self);
            self.report_match(recognizer);
            let matched_symbol = recognizer.get_current_token();
            return Some(matched_symbol);
        }
        None
    }

    fn get_missing_symbol<
        T: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
    >(
        &self,
        recognizer: &mut T,
    ) -> <T::TF as TokenFactory<'input>>::Tok {
        let expected = self.get_expected_tokens(recognizer);
        let expected_token_type = expected.get_min().unwrap_or(TOKEN_INVALID_TYPE);

        let curr = &recognizer.get_current_token().data;

        Box::new(AntlrToken::new(
            FileTokPos::new_meta_c(Tok::getFromTType(expected_token_type), curr),
            -1,
        ))
        // Token::to_owned(token.borrow())
        // .modify_with(|it| it.text = token_text)
    }

    fn get_expected_tokens<
        T: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
    >(
        &self,
        recognizer: &T,
    ) -> IntervalSet {
        recognizer.get_expected_tokens()
    }

    fn get_token_error_display<T: Token + ?Sized>(&self, t: &T) -> String {
        let text = t.get_text().to_display();
        self.escape_ws_and_quote(&text)
    }

    fn escape_ws_and_quote(&self, s: &str) -> String {
        format!("'{}'", Self::escape_whitespaces(s, false))
    }

    fn get_error_recovery_set<
        T: Parser<'input, Node = Ctx, TF = AntlrLexerWrapperFactory<'input, Tok>>,
    >(
        &self,
        recognizer: &T,
    ) -> IntervalSet {
        let atn = recognizer.get_interpreter().atn();
        let mut ctx = Some(recognizer.get_parser_rule_context().clone());
        let mut recover_set = IntervalSet::new();
        while let Some(c) = ctx {
            if c.get_invoking_state() < 0 {
                break;
            }

            let invoking_state = atn.states[c.get_invoking_state() as usize].as_ref();
            let tr = invoking_state.get_transitions().first().unwrap().as_ref();
            let tr = tr.cast::<RuleTransition>();
            let follow = atn.next_tokens(atn.states[tr.follow_state].as_ref());
            recover_set.add_set(follow);
            ctx = c.get_parent_ctx();
        }
        recover_set.remove_one(TOKEN_EPSILON);
        return recover_set;
    }

    fn consume_until(&mut self, recognizer: &mut Recognizer, set: &IntervalSet) {
        let mut ttype = recognizer.get_input_stream_mut().la(1);
        while ttype != TOKEN_EOF && !set.contains(ttype) {
            recognizer.consume(self);
            ttype = recognizer.get_input_stream_mut().la(1);
        }
    }
}

impl<
        'a,
        Ctx: ParserNodeType<'a>,
        Tok: Clone + Debug + HasEOF + Display + 'static,
        T: Parser<'a, Node = Ctx, TF = AntlrLexerWrapperFactory<'a, Tok>> + 'a,
    > ErrorStrategy<'a, T> for LexerWrapperErrorStrategy<'a, Ctx, Tok, T>
{
    fn reset(&mut self, recognizer: &mut T) {
        self.end_error_condition(recognizer);
    }

    fn recover_inline(
        &mut self,
        recognizer: &mut T,
    ) -> Result<<T::TF as TokenFactory<'a>>::Tok, ANTLRError> {
        let t = self
            .single_token_deletion(recognizer)
            .map(std::borrow::ToOwned::to_owned);
        if let Some(t) = t {
            recognizer.consume(self);
            return Ok(t);
        }

        if self.single_token_insertion(recognizer) {
            return Ok(self.get_missing_symbol(recognizer));
        }

        if let Some(next_tokens_ctx) = &self.next_tokens_ctx {
            Err(ANTLRError::InputMismatchError(
                InputMisMatchError::with_state(
                    recognizer,
                    self.next_tokens_state,
                    next_tokens_ctx.clone(),
                ),
            ))
        } else {
            Err(ANTLRError::InputMismatchError(InputMisMatchError::new(
                recognizer,
            )))
        }
        //        Err(ANTLRError::IllegalStateError("aaa".to_string()))
    }

    fn recover(&mut self, recognizer: &mut T, _e: &ANTLRError) -> Result<(), ANTLRError> {
        if self.last_error_index == recognizer.get_current_token().get_token_index()
            && self.last_error_states.is_some()
            && self
                .last_error_states
                .as_ref()
                .unwrap()
                .contains(recognizer.get_state())
        {
            recognizer.consume(self);
        }

        self.last_error_index = recognizer.get_current_token().get_token_index();
        self.last_error_states
            .get_or_insert(IntervalSet::new())
            .add_one(recognizer.get_state());
        let follow_set = self.get_error_recovery_set(recognizer);
        self.consume_until(recognizer, &follow_set);
        Ok(())
    }

    fn sync(&mut self, recognizer: &mut T) -> Result<(), ANTLRError> {
        if self.in_error_recovery_mode(recognizer) {
            return Ok(());
        }
        let next = recognizer.get_input_stream_mut().la(1);
        let state =
            recognizer.get_interpreter().atn().states[recognizer.get_state() as usize].as_ref();

        let next_tokens = recognizer.get_interpreter().atn().next_tokens(state);

        if next_tokens.contains(next) {
            self.next_tokens_state = ATNSTATE_INVALID_STATE_NUMBER;
            self.next_tokens_ctx = None;
            return Ok(());
        }

        if next_tokens.contains(TOKEN_EPSILON) {
            if self.next_tokens_ctx.is_none() {
                self.next_tokens_state = recognizer.get_state();
                self.next_tokens_ctx = Some(recognizer.get_parser_rule_context().clone());
            }
            return Ok(());
        }

        match state.get_state_type_id() {
            ATNSTATE_BLOCK_START
            | ATNSTATE_PLUS_BLOCK_START
            | ATNSTATE_STAR_BLOCK_START
            | ATNSTATE_STAR_LOOP_ENTRY => {
                if self.single_token_deletion(recognizer).is_none() {
                    return Err(ANTLRError::InputMismatchError(InputMisMatchError::new(
                        recognizer,
                    )));
                }
            }
            ATNSTATE_PLUS_LOOP_BACK | ATNSTATE_STAR_LOOP_BACK => {
                self.report_unwanted_token(recognizer);
                let mut expecting = recognizer.get_expected_tokens();
                expecting.add_set(&self.get_error_recovery_set(recognizer));
                self.consume_until(recognizer, &expecting);
            }
            _ => panic!("invalid ANTState type id"),
        }

        Ok(())
    }

    fn in_error_recovery_mode(&mut self, _recognizer: &mut T) -> bool {
        self.error_recovery_mode
    }

    fn report_error(&mut self, recognizer: &mut T, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let msg = match e {
            ANTLRError::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRError::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            ANTLRError::PredicateError(e) => self.report_failed_predicate(recognizer, e),
            _ => e.to_string(),
        };
        self.errorList
            .lock()
            .unwrap()
            .push(e.get_offending_token().map_or(
                CompileError::from_at(&msg, self.file.clone(), 0, Some(0)),
                |x| {
                    let idx = x.get_token_index();
                    let tok = recognizer.get_input_stream().get(idx);
                    CompileError::from_preTo(&msg, &tok.data)
                },
            ));
    }

    fn report_match(&mut self, recognizer: &mut T) {
        self.end_error_condition(recognizer);
        //println!("matched token succesfully {}", recognizer.get_input_stream().la(1))
    }
}
