use sys;

/// A pass manager.
pub struct PassManager(sys::PassManagerRef);

impl PassManager
{
    /// Creates a new pass manager.
    pub fn new() -> Self {
        let inner = unsafe { sys::LLVMRustCreateLegacyPassManager() };
        PassManager(inner)
    }

    /// Creates a pass manager from a reference to an LLVM pass manager.
    pub unsafe fn from_ref(inner: sys::PassManagerRef) -> Self {
        PassManager(inner)
    }
}

impl Drop for PassManager
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
        PassManager::new();
    }
}
