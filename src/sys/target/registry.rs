use TargetRef;
use libc;

cpp! {
    #include "llvm/Support/TargetRegistry.h"
    #include "llvm/Support/TargetSelect.h"

    pub fn LLVMRustInitializeAllTargets() {
        llvm::InitializeAllTargetInfos();
        llvm::InitializeAllTargets();
        llvm::InitializeAllTargetMCs();
        llvm::InitializeAllAsmParsers();
        llvm::InitializeAllAsmPrinters();
    }

    pub fn LLVMRustTargetRegistryTargets(dest: *mut TargetRef as "llvm::Target**",
                                         max_count: libc::size_t as "size_t")
        -> libc::size_t as "size_t" {
        int cur_count = 0;

        for (const llvm::Target &target : llvm::TargetRegistry::targets()) {
            assert(cur_count <= max_count && "too many targets to fit in the list");
            dest[cur_count] = const_cast<llvm::Target*>(&target);

            cur_count += 1;
        }

        return cur_count;;
    }
}
