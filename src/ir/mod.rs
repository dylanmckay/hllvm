//! The LLVM intermediate representation.

pub use self::context::Context;
pub use self::module::Module;
pub use self::ty::Type;
pub use self::attribute::Attribute;

pub mod context;
pub mod module;
pub mod ty;
pub mod attribute;