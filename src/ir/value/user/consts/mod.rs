//! Compile-time constant values.

pub use self::int::ConstantInt;
pub use self::fp::ConstantFP;
pub use self::blockaddr::BlockAddress;
pub use self::aggzero::ConstantAggregateZero;
pub use self::array::ConstantArray;
pub use self::vector::ConstantVector;
pub use self::ptrnull::ConstantPointerNull;
pub use self::strukt::ConstantStruct;
pub use self::undef::UndefValue;
pub use self::global::*;

pub mod int;
pub mod fp;
pub mod blockaddr;
pub mod aggzero;
pub mod array;
pub mod vector;
pub mod ptrnull;
pub mod strukt;
pub mod undef;
pub mod global;

use ir::User;

/// A value that is constant at compile time.
pub struct Constant<'ctx>(User<'ctx>);
impl_subtype!(Constant => User);
