use std::{
    collections::HashMap,
    fmt::Display,
    ops::Add,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use bumpalo::Bump;

struct ProtectedBump {
    bump: Bump,
}
unsafe impl Sync for ProtectedBump {}

struct StringRefMap {
    stringsBump: &'static ProtectedBump,
    map: HashMap<&'static str, StringRef>,
}
unsafe impl Sync for StringRefMap {}

lazy_static! {
    static ref STRING_REF_MAP: Arc<Mutex<StringRefMap>> = {
        lazy_static! {
            static ref PROTECTED_BUMP: ProtectedBump = ProtectedBump { bump: Bump::new() };
        }
        Arc::new(Mutex::new(StringRefMap {
            stringsBump: &PROTECTED_BUMP,
            map: HashMap::new(),
        }))
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct StringRefImpl {
    ptr: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringRef {
    ptr: &'static StringRefImpl,
}

impl std::hash::Hash for StringRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let string: *const str = self.ptr.ptr;
        let void: *const () = string.cast::<()>();
        void.hash(state);
    }
}

impl Add for StringRef {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.ptr.ptr.to_string() + rhs.ptr.ptr).to_StringRef()
    }
}
unsafe impl Sync for StringRef {}

impl Display for StringRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl StringRef {
    pub fn from_str(str: &str) -> Self {
        let mut map = STRING_REF_MAP.lock().unwrap();

        #[allow(clippy::option_if_let_else)]
        if let Some(v) = map.map.get(str) {
            *v
        } else {
            let ptr_str = map.stringsBump.bump.alloc_str(str);
            let ptr_refStr = map.stringsBump.bump.alloc(StringRefImpl { ptr: ptr_str });
            let me = Self { ptr: ptr_refStr };
            map.map.insert(ptr_str, me);
            me
        }
    }
}

impl AsRef<str> for StringRef {
    fn as_ref(&self) -> &str {
        self.ptr.ptr
    }
}

pub trait ToStringRef {
    fn to_StringRef(&self) -> StringRef;
}

impl ToStringRef for String {
    fn to_StringRef(&self) -> StringRef {
        StringRef::from_str(self.as_str())
    }
}

impl ToStringRef for &str {
    fn to_StringRef(&self) -> StringRef {
        StringRef::from_str(self)
    }
}
