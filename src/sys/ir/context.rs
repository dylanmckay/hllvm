use ContextRef;

extern "C" {
    pub fn LLVMRustCreateContext() -> ContextRef;
    pub fn LLVMRustDestroyContext(_: ContextRef);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_and_destroy() {
        let ctx = unsafe { LLVMRustCreateContext() };
        unsafe { LLVMRustDestroyContext(ctx) };
    }
}
