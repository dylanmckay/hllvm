//! The LLVM intermediate representation.

pub use self::context::Context;
pub use self::module::Module;
pub use self::ty::{Type, FunctionType, IntegerType};
pub use self::attribute::Attribute;
pub use self::value::Value;

pub mod context;
pub mod module;
pub mod ty;
pub mod attribute;
pub mod value;
