use SafeWrapper;
use sys;

/// An LLVM context.
pub struct Context(sys::ContextRef);

impl Context
{
    pub fn new() -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateContext();
            Context::from_inner(inner)
        }
    }
}

impl SafeWrapper for Context
{
    type Inner = sys::ContextRef;

    unsafe fn from_inner(inner: sys::ContextRef) -> Self { Context(inner) }
    fn inner(&self) -> sys::ContextRef { self.0 }
}

impl Drop for Context
{
    fn drop(&mut self) {
        unsafe {
            sys::LLVMRustDestroyContext(self.0);
        }
    }
}
