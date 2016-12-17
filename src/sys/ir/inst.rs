use {ContextRef, ValueRef};

extern "C" {
    pub fn LLVMRustCreateReturnInst(_: ContextRef,
                                    ret_val: ValueRef) -> ValueRef;
}
