pub use self::opcode::*;
pub use self::ty::*;
pub use self::value::*;
pub use self::context::*;
pub use self::module::*;
pub use self::attributes::*;
pub use self::block::*;
pub use self::function::*;
pub use self::passes::*;

pub mod opcode;
pub mod ty;
pub mod value;
pub mod context;
pub mod module;
pub mod attributes;
pub mod block;
pub mod function;
pub mod passes;

/// An enumeration for the kinds of linkage for global values.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum Linkage
{
    /// Externally visible function.
    External = 0,
    /// Available for inspection, not emission.
    AvailableExternally,
    /// Keep one copy of function when linking (inline).
    LinkOnceAny,
    /// Same as `LinkOnceAny`, but only replaced by something equivalent.
    LinkOnceODR,
    /// Keep one copy of named function when linking (weak).
    WeakAny,
    /// Same as `WeakAny`, but only replaced by something equivalent.
    WeakODR,
    /// Special purpose, only applies to global arrays.
    Appending,
    /// Rename collisions when linking (static functions).
    Internal,
    /// Like `Internal`, but omit from symbol table.
    Private,
    ExternalWeak,
    /// Tentative definitions.
    Common,
}

/// Atomic ordering.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum AtomicOrdering
{
    NotAtomic = 0,
    Unordered = 1,
    Monotonic = 2,
    // 3 is not implemented yet.
    Acquire = 4,
    Release = 5,
    AcquireRelease = 6,
    SequentiallyConsistent = 7,
}

/// Synchronization scope.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum SynchronizationScope
{
    SingleThread = 0,
    CrossThread = 1,
}

/// Thread local mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum ThreadLocalMode
{
    NotThreadLocal = 0,
    GeneralDynamic,
    LocalDynamic,
    InitialExec,
    LocalExec,
}

/// Float predicate kind.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum FloatPredicateKind
{
    False = 0,
    OrderedAndEqual = 1,
    OrderedGreaterThan = 2,
    OrderedGreaterThanOrEqual = 3,
    OrderedLessThan = 4,
    OrderedLessThanOrEqual = 5,
    OrderedUnequal = 6,
    Ordered = 7,
    Unordered = 8,
    UnorderedOrEqual = 9,
    UnorderedOrGreaterThan = 10,
    UnorderedOrGreaterThanOrEqual = 11,
    UnorderedOrLessThan = 12,
    UnorderedOrLessThanOrEqual = 13,
    UnorderedOrNotEqual = 14,
    True = 15,
}

/// Integer predicate kind.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum IntegerPredicateKind
{
    Equal = 32,
    NotEqual = 33,
    UnsignedGreaterThan = 34,
    UnsignedGreaterThanOrEqual = 35,
    UnsignedLessThan = 36,
    UnsignedLessThanOrEqual = 37,
    SignedGreaterThan = 38,
    SignedGreaterThanOrEqual = 39,
    SignedLessThan = 40,
    SignedLessThanOrEqual = 41,
}

#[cfg(test)]
mod test
{
    use ir;
    use std::mem;
    use libc;

    #[test]
    fn linkage_ints_are_correct() {
        assert_eq!(ir::Linkage::External as u32, 0);
        assert_eq!(ir::Linkage::Private as u32, 8);
    }

    #[test]
    fn linkage_is_c_unsigned() {
        assert_eq!(mem::size_of::<ir::Linkage>(), mem::size_of::<libc::c_uint>());
    }
}
