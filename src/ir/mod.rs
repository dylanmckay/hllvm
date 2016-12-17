//! The LLVM intermediate representation.

pub use self::context::Context;
pub use self::module::Module;
pub use self::ty::Type;

pub mod context;
pub mod module;
pub mod ty;
