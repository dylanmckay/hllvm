use ir;
use sys;

/// A pass manager.
pub struct Manager(sys::PassManagerRef);

impl Manager
{
    /// Creates a new pass manager.
    pub fn new() -> Self {
        let inner = unsafe { sys::LLVMRustCreateLegacyPassManager() };
        Manager(inner)
    }

    /// Creates a pass manager from a reference to an LLVM pass manager.
    pub unsafe fn from_ref(inner: sys::PassManagerRef) -> Self {
        Manager(inner)
    }

    /// Run passes on a module.
    pub fn run<'ctx>(&self, module: ir::Module<'ctx>) -> ir::Module<'ctx> {
        unsafe {
            sys::LLVMRustLegacyPassManagerRun(self.0, module.inner());
        }
        module
    }

    pub fn inner(&self) -> sys::PassManagerRef { self.0 }
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
