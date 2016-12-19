use SafeWrapper;
use ir;
use sys;

/// A pass manager.
pub struct Manager(sys::PassManagerRef);

impl Manager
{
    /// Creates a new pass manager.
    pub fn new() -> Self {
        unsafe {
            let inner = sys::LLVMRustCreateLegacyPassManager();
            Manager::from_inner(inner)
        }
    }

    /// Run passes on a module.
    pub fn run<'ctx>(&self, module: ir::Module<'ctx>) -> ir::Module<'ctx> {
        unsafe {
            sys::LLVMRustLegacyPassManagerRun(self.0, module.inner());
        }
        module
    }
}

impl SafeWrapper for Manager
{
    type Inner = sys::PassManagerRef;

    unsafe fn from_inner(inner: sys::PassManagerRef) -> Self { Manager(inner) }
    fn inner(&self) -> sys::PassManagerRef { self.0 }
}

impl Drop for Manager
{
    fn drop(&mut self) {
        unsafe {
            sys::LLVMRustDestroyLegacyPassManager(self.0);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_pm() {
        Manager::new();
    }
}
