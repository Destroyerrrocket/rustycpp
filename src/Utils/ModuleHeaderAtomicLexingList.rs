use std::sync::{atomic::AtomicUsize, Mutex};

/// This is an overly specific class to handle the atomic lexing of the module
/// headers in parallel
///
/// This is a tad complicated, you see: we want to lex the module headers in
/// parallel, but we also want to transform includes to these module headers to
/// imports. This becomes a major issue during lexing, where macros from one
/// header can be used in another. This means that while lexing a header, we
/// might need to stop to lex another (We can't just ask the users to provide us
/// the dependency graph of headers, that's like, our job). So, what do we do if
/// two headers depend in another header? We could let one parse that one, and
/// the other one could wait. This would make detecting include loops
/// "relatively" easy, but this can and will introduce bottlenecks. So, instead,
/// We will start lexing whichever header is available next. Unfortunately, this
/// means that we also need to keep track of which headers are not lexed, which
/// are being lexed, which headers are stuck parsing another header, and which
/// ones are done.
///
/// This class allows parsers to start another parser

pub struct ModuleHeaderAtomicLexingList {
    available: Mutex<Vec<Box<dyn Fn() + Send>>>,
    lockedThreads: AtomicUsize,
    totalThreads: usize,
}

impl std::fmt::Debug for ModuleHeaderAtomicLexingList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleHeaderAtomicLexingList")
            .field("available", &self.available.lock().unwrap().len())
            .finish_non_exhaustive()
    }
}

impl ModuleHeaderAtomicLexingList {
    pub fn new(totalThreads: usize) -> Self {
        Self {
            available: Mutex::new(Vec::new()),
            lockedThreads: AtomicUsize::new(0),
            totalThreads,
        }
    }

    pub fn push(&self, f: Vec<Box<dyn Fn() + Send>>) {
        self.available.lock().unwrap().extend(f);
    }

    pub fn pop(&self) -> Option<Box<dyn Fn() + Send>> {
        self.available.lock().unwrap().pop()
    }

    pub fn markThreadLocked(&self) -> bool {
        let prevValue = self
            .lockedThreads
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if prevValue + 1 == self.totalThreads {
            self.markThreadUnlocked();
        }
        prevValue + 1 == self.totalThreads
    }

    pub fn markThreadUnlocked(&self) {
        self.lockedThreads
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
    }
}
