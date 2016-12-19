use {PassManagerRef, ModuleRef};

cpp! {
    #include "llvm/IR/LegacyPassManager.h"

    pub fn LLVMRustCreateLegacyPassManager()
        -> PassManagerRef as "llvm::legacy::PassManager*" {
        return new llvm::legacy::PassManager();
    }

    pub fn LLVMRustDestroyLegacyPassManager(pm: PassManagerRef as "llvm::legacy::PassManager*") {
        delete pm;
    }

    pub fn LLVMRustLegacyPassManagerRun(pm: PassManagerRef as "llvm::legacy::PassManager*",
                                        m: ModuleRef as "llvm::Module*") -> bool as "bool" {
        return pm->run(*m);
    }
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
