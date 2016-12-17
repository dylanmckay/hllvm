# LLVM C FFI library

## Conventions

## Layout

The project attempts to closely follow the LLVM C++ header structure.

If there is an upstream `llvm/IR/Value.h` header, we will define the following files here:

* `ir/value.cpp` - The actual FFI function definitions.
* `ir/value.rs`  - FFI declarations for that file.

Each `.cpp` file should have an associated `.rs` file.

All FFI functions declared in Rust should be glob imported into the crate root.

## Code

All functions will be prefixed with `LLVMRust` and will be exported as
functions with the C ABI and no name mangling.

### Constructor/destructor functions

These should be named like `LLVMRustCreate<ClassName>` or `LLVMRustDestroy<ClassName>`.

Examples:

* `LLVMRustCreateContext`
* `LLVMRustDestroyContext`
* `LLVMRustCreateValue`

# Methods and static functions

If we have a `get` function on `Value`, the FFI function should look like

```rust
LLVMRustValueGet
```

Static functions are not capitalized differently than standard methods.
