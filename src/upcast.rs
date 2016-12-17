/// A trait for things that can be upcasted.
pub trait Upcast
{
    type Parent;

    fn upcast_ref(&self) -> &Self::Parent;
    fn upcast_mut(&mut self) -> &mut Self::Parent;
    fn upcast(self) -> Self::Parent;
}

/// Implements `Upcast` for a type that is owned by an LLVM context.
macro_rules! impl_upcast {
    ($ty:ident => $parent:ident) => {
        impl<'ctx> $crate::upcast::Upcast for $ty<'ctx> {
            type Parent = $parent<'ctx>;

            fn upcast_ref(&self) -> &Self::Parent { &self.0 }
            fn upcast_mut(&mut self) -> &mut Self::Parent { &mut self.0 }
            fn upcast(self) -> Self::Parent { self.0 }
        }

        impl<'a> AsRef<$parent<'a>> for $ty<'a>
        {
            fn as_ref(&self) -> &$parent<'a> { &self.0 }
        }
    }
}
