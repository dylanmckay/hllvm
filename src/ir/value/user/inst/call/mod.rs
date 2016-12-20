use SafeWrapper;
use ir::{User, Instruction, Value, Function};
use sys;

use std::ffi;

/// A instruction which calls a fuction.
pub struct CallInst<'ctx>(Instruction<'ctx>);

impl<'ctx> CallInst<'ctx>
{
    /// Creates a new call instruction.
    pub fn new(func: &Function,
               args: &[&Value],
               // FIXME: remove this argument
               name: &str) -> Self {
        let args: Vec<_> = args.iter().map(|v| v.inner()).collect();
        let name = ffi::CString::new(name).unwrap();

        unsafe {
            let inner = sys::LLVMRustCreateCallInst(func.inner(), &args, name.as_ptr());
            CallInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_upcast!(CallInst => Instruction);
