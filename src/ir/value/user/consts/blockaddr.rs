//! Basic block address.

use SafeWrapper;
use ir::{Constant, Block, User};
use sys;

/// The address of a basic block.
pub struct BlockAddress<'ctx>(Constant<'ctx>);
impl_subtype!(BlockAddress => Constant);

impl<'ctx> BlockAddress<'ctx>
{
    /// Creates a new block address from a basic block.
    pub fn new(block: &Block) -> Self {
        unsafe {
            let inner = sys::LLVMRustBlockAddressGet(block.inner());
            wrap_value!(inner => User => Constant => BlockAddress)
        }
    }
}
