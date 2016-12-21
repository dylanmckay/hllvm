pub use self::atomiccmp::AtomicCmpXchgInst;
pub use self::store::StoreInst;
pub use self::gep::GetElementPtrInst;
pub use self::extract_elem::ExtractElementInst;
pub use self::insert_elem::InsertElementInst;
pub use self::insert_value::InsertValueInst;
pub use self::select::SelectInst;
pub use self::fence::FenceInst;
pub use self::landingpad::LandingPadInst;
pub use self::binop::BinaryOperatorInst;
pub use self::catchpad::CatchPadInst;
pub use self::cleanuppad::CleanupPadInst;
pub use self::call::*;
pub use self::cmp::*;
pub use self::terminator::*;
pub use self::unary::*;

pub mod atomiccmp;
pub mod store;
pub mod gep;
pub mod extract_elem;
pub mod insert_elem;
pub mod insert_value;
pub mod select;
pub mod fence;
pub mod landingpad;
pub mod binop;
pub mod catchpad;
pub mod cleanuppad;
pub mod call;
pub mod cmp;
pub mod terminator;
pub mod unary;

use ir::User;

pub struct Instruction<'ctx>(User<'ctx>);
impl_upcast!(Instruction => User);

macro_rules! define_unimplemented_inst {
    ($name:ident) => {
        /// Unimplemented instruction.
        pub struct $name<'ctx>(Instruction<'ctx>);

        impl_upcast!($name => Instruction);
    }
}

define_unimplemented_inst!(AtomicRMWInst);
define_unimplemented_inst!(ShuffleVectorInst);
define_unimplemented_inst!(CatchSwitchInst);
