# LLVM C FFI library

## Conventions

All functions will be prefixed with `LLVMRust` and will be exported as
functions with the C ABI and no name mangling.

## Constructor/destructor functions

These should be named like `LLVMRustCreate<ClassName>` or `LLVMRustDestroy<ClassName>`.

Examples:

* `LLVMRustCreateContext`
* `LLVMRustDestroyContext`
* `LLVMRustCreateValue`
