use SafeWrapper;
use ir::{Value, Context, Function, Instruction};
use Upcast;
use sys;

use std::{ptr, ffi};

/// A basic block.
pub struct Block<'ctx>(Value<'ctx>);

impl<'ctx> Block<'ctx>
{
    /// Creates a new basic block.
    pub fn new(context: &Context,
               name: Option<&str>,
               parent: Option<&Function>,
               insert_before: Option<&Self>) -> Self {
        let insert_before = insert_before.map_or(ptr::null_mut(), |b| b.0.inner());
        let name = ffi::CString::new(name.unwrap_or("")).unwrap();

        unsafe {
            Block(Value::from_inner(sys::LLVMRustBasicBlockCreate(context.inner(),
                name.as_ptr(),
                parent.map_or(ptr::null_mut(), |f| f.inner()),
                insert_before)))
        }
    }

    /// Adds an instruction to a basic block.
    pub fn append(&mut self, inst: &Instruction) {
        unsafe {
            sys::LLVMRustInstructionAppend(inst.upcast_ref().inner(), self.0.inner());
        }
    }
}

impl_upcast!(Block => Value);
