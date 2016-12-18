use {PassManagerRef, ModuleRef};

extern "C" {
    pub fn LLVMRustCreateLegacyPassManager() -> PassManagerRef;
    pub fn LLVMRustDestroyLegacyPassManager(_: PassManagerRef);

    pub fn LLVMRustLegacyPassManagerRun(_: PassManagerRef,
                                        m: ModuleRef) -> bool;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_and_destroy_pms() {
        unsafe {
            let pm = LLVMRustCreateLegacyPassManager();
            LLVMRustDestroyLegacyPassManager(pm);
        }
    }
}
