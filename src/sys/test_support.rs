//! Helpers for writing tests.

use ContextRef;

/// An LLVM  context.
pub struct Context {
    pub inner: ContextRef,
}

impl Context {
    pub fn new() -> Self {
        Context {
            inner: unsafe { ::LLVMRustCreateContext() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ::LLVMRustDestroyContext(self.inner) };
    }
}
