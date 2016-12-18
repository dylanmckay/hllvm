use PassManagerRef;

extern "C" {
    pub fn LLVMRustCreateLegacyPassManager() -> PassManagerRef;
    pub fn LLVMRustDestroyLegacyPassManager(_: PassManagerRef);
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
