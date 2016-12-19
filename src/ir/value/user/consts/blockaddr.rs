//! Basic block address.

use SafeWrapper;
use ir::{Constant, Value, Block, User};
use sys;

/// The address of a basic block.
pub struct BlockAddress<'ctx>(Constant<'ctx>);
impl_upcast!(BlockAddress => Constant);

impl<'ctx> BlockAddress<'ctx>
{
    /// Creates a new block address from a basic block.
    pub fn new(block: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustBlockAddressGet(block.inner());
            BlockAddress(Constant(User(Value::from_inner(inner))))
        }
    }
}
