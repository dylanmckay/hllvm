use {ContextRef, ValueRef};

extern "C" {
    pub fn LLVMRustInstructionInsertAfter(inst: ValueRef,
                                          after: ValueRef);

    pub fn LLVMRustInstructionAppend(inst: ValueRef, block: ValueRef);

    pub fn LLVMRustCreateReturnInst(_: ContextRef,
                                    ret_val: ValueRef) -> ValueRef;
}
