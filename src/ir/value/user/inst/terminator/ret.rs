use SafeWrapper;
use ir::{User, Context, Instruction, Value, TerminatorInst};
use sys;

use std::ptr;

pub struct ReturnInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> ReturnInst<'ctx>
{
    pub fn new(ret_val: Option<&Value>,
               context: &Context) -> Self {
        let ret_val = ret_val.map(Value::inner).unwrap_or(ptr::null_mut());

        unsafe {
            let inner = sys::LLVMRustCreateReturnInst(context.inner(), ret_val);
            wrap_value!(inner => User => Instruction => TerminatorInst => ReturnInst)
        }
    }
}

impl_subtype!(ReturnInst => TerminatorInst);
