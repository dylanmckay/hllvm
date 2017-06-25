//! Sequential types.

pub use self::array::ArrayType;
pub use self::pointer::PointerType;
pub use self::vector::VectorType;

pub mod array;
pub mod pointer;
pub mod vector;

use ir::CompositeType;

/// A sequential type.
pub struct SequentialType<'ctx>(CompositeType<'ctx>);
impl_subtype!(SequentialType => CompositeType);
