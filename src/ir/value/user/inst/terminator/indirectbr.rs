use SafeWrapper;
use ir::{User, Instruction, Value, TerminatorInst, Block};
use sys;

/// An indirect branch.
pub struct IndirectBrInst<'ctx>(TerminatorInst<'ctx>);

impl<'ctx> IndirectBrInst<'ctx>
{
    /// Creates a new indirect branch.
    pub fn new(address: &Value,
               destinations: &[&Block]) -> Self {
        let mut br = unsafe {
            let inner = sys::LLVMRustCreateIndirectBrInst(address.inner());
            wrap_value!(inner => User => Instruction => TerminatorInst => IndirectBrInst)
        };

        for dest in destinations { br.add_destination(dest) }

        br
    }

    /// Adds a possible destination block.
    pub fn add_destination(&mut self, block: &Block) {
        unsafe { sys::LLVMRustIndirectBrInstAddDestination(self.inner(), block.inner()) }
    }
}

impl_subtype!(IndirectBrInst => TerminatorInst);
