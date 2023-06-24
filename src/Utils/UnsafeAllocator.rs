use std::cell::UnsafeCell;

/**
 * This is a wrapper around a bumpalo allocator that will pretend that it has a static lifetime. This is not true,
 * but if only used while creating the AST, and as long as the `AstTu` has a copy alive, nothing bad will happen.
 *
 * This is basically needed because otherwise the creation of the AST would be very annoying, as the lifetimes
 * would constantly be nagging me.
*/
pub struct UnsafeAllocator {
    alloc: UnsafeCell<bumpalo::Bump>,
}

impl UnsafeAllocator {
    pub fn new() -> Self {
        Self {
            alloc: UnsafeCell::new(bumpalo::Bump::new()),
        }
    }

    pub fn alloc(&self) -> &'static bumpalo::Bump {
        unsafe { UnsafeCell::get(&self.alloc).as_ref().unwrap_unchecked() }
    }
}
