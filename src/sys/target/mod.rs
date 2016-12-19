pub use self::registry::*;
pub use self::target::*;

pub mod registry;
pub mod target;

/// Specifies a file type.
/// Analgous to `llvm::CodeGenFileType`
#[repr(C)]
pub enum FileType {
    /// Asn assembly file.
    Assembly = 0,
    /// An object file.
    Object   = 1,
}
