//! Trait for types which wrap unsafe LLVM internals.

/// A type which wraps an underlying LLVM FFI type.
pub trait SafeWrapper : Sized
{
    /// The inner type.
    type Inner: Sized;

    /// Creates a new instance of the type from an underlying value.
    unsafe fn from_inner(inner: Self::Inner) -> Self;

    /// Gets the underlying value.
    fn inner(&self) -> Self::Inner;
}
