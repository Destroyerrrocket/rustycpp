use antlr_rust::token::Token;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::{TidAble, TokenSource};

use std::borrow::Cow;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

use super::structs::FilePreTokPos;

pub trait HasEOF {
    fn getEOF() -> Self;
    fn getInvalid() -> Self;
}

#[derive(Debug, Clone)]
pub struct AntlrToken<T: Clone + Debug + HasEOF> {
    data: FilePreTokPos<T>,
    index: Arc<AtomicIsize>,
}

impl<T: Clone + Debug + HasEOF> AntlrToken<T> {
    pub fn new(data: FilePreTokPos<T>, index: isize) -> Self {
        Self {
            data,
            index: Arc::new(AtomicIsize::new(index)),
        }
    }
}

impl<T: Clone + Debug + HasEOF> std::fmt::Display for AntlrToken<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.tokPos.tok.fmt(f)
    }
}

impl<T: Clone + Debug + HasEOF + 'static> Token for AntlrToken<T> {
    type Data = FilePreTokPos<T>;

    fn get_channel(&self) -> isize {
        antlr_rust::token::TOKEN_DEFAULT_CHANNEL
    }

    fn get_start(&self) -> isize {
        self.data.tokPos.start.try_into().unwrap()
    }

    fn get_stop(&self) -> isize {
        self.data.tokPos.end.try_into().unwrap()
    }

    fn get_line(&self) -> isize {
        isize::try_from(self.data.file.getRowColumn(self.data.tokPos.start).0).unwrap()
    }

    fn get_column(&self) -> isize {
        isize::try_from(self.data.file.getRowColumn(self.data.tokPos.start).1).unwrap()
    }

    fn set_text(&mut self, _text: <Self::Data as ToOwned>::Owned) {
        unimplemented!()
    }

    fn get_token_index(&self) -> isize {
        self.index.load(Ordering::Relaxed)
    }

    fn set_token_index(&self, v: isize) {
        self.index.store(v, Ordering::Relaxed);
    }

    fn to_owned(&self) -> antlr_rust::token::OwningToken {
        unimplemented!()
    }

    fn get_token_type(&self) -> isize {
        unsafe { *std::ptr::addr_of!(self.data.tokPos.tok).cast::<isize>() }
    }

    fn get_text(&self) -> &Self::Data {
        &self.data
    }
}

#[derive(Debug)]
pub struct AntlrLexerWrapperFactory<'a, U: Clone + Debug + HasEOF> {
    _phantom: &'a PhantomData<U>,
}

impl<'a, U: Clone + Debug + HasEOF + 'static> TokenFactory<'a> for AntlrLexerWrapperFactory<'a, U> {
    type Inner = AntlrToken<U>;

    type Tok = Box<Self::Inner>;

    type Data = FilePreTokPos<U>;

    type From = Cow<'static, str>;

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
        todo!()
    }

    fn create_invalid() -> Self::Tok {
        Box::new(AntlrToken::new(
            FilePreTokPos::new_meta(U::getInvalid()),
            -1,
        ))
    }

    fn get_data(_: Self::From) -> std::borrow::Cow<'static, Self::Data> {
        todo!()
    }
}

unsafe impl<'a, T: Clone + Debug + HasEOF + 'static> TidAble<'a>
    for AntlrLexerWrapperFactory<'a, T>
{
    type Static = ();
}

#[derive(Debug)]
pub struct AntlrLexerWrapper<'a, T: Clone + Debug + HasEOF> {
    pd: &'a PhantomData<AntlrToken<T>>,
    tokens: VecDeque<FilePreTokPos<T>>,
    idx: isize,
    file: String,
}

impl<'a, T: Clone + Debug + HasEOF> AntlrLexerWrapper<'a, T> {
    pub const fn new(tokens: VecDeque<FilePreTokPos<T>>, file: String) -> Self {
        Self {
            pd: &PhantomData,
            tokens,
            idx: 0,
            file,
        }
    }
}

impl<'a, T: Clone + Debug + HasEOF + 'static> TokenSource<'a> for AntlrLexerWrapper<'a, T> {
    type TF = AntlrLexerWrapperFactory<'a, T>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'a>>::Tok {
        if let Some(tok) = self.tokens.pop_front() {
            self.idx += 1;
            Box::new(AntlrToken::new(tok, self.idx))
        } else {
            Box::new(AntlrToken::new(
                FilePreTokPos::new_meta(T::getEOF()),
                self.idx,
            ))
        }
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        unimplemented!()
    }

    fn get_source_name(&self) -> String {
        self.file.clone()
    }

    fn get_token_factory(&self) -> &'a Self::TF {
        unimplemented!()
    }
}

unsafe impl<'a, T: Clone + Debug + HasEOF + 'static> TidAble<'a> for AntlrLexerWrapper<'a, T> {
    type Static = ();
}
