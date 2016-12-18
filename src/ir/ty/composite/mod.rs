//! Composite types.

pub use self::strukt::StructType;
pub use self::seq::*;

pub mod strukt;
pub mod seq;

use ir::Type;

/// A composite type.
pub struct CompositeType<'ctx>(Type<'ctx>);
impl_upcast!(CompositeType => Type);
