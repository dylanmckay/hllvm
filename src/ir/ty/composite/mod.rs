pub use self::strukt::StructType;

pub mod strukt;

use ir::Type;

/// A composite type.
pub struct CompositeType<'ctx>(Type<'ctx>);
impl_upcast!(CompositeType => Type);
