//! The LLVM intermediate representation.

pub use self::context::Context;
pub use self::module::Module;
pub use self::attribute::Attribute;
pub use self::ty::*;
pub use self::value::*;

// Reexports
pub use sys::{Linkage, SynchronizationScope, AtomicOrdering,
              ThreadLocalMode};
pub use sys::ir::opcode::*;

pub mod context;
pub mod module;
pub mod ty;
pub mod attribute;
pub mod value;
