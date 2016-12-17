use sys;

/// An LLVM context.
pub struct Context
{
    inner: sys::ContextRef,
}

impl Context
{
    pub fn new() -> Self {
        Context {
            inner: unsafe { sys::LLVMRustCreateContext() },
        }
    }

    /// Get a reference to the inner context reference.
    pub fn inner(&self) -> sys::ContextRef { return self.inner }
}
