/// A trait for things that can be upcasted.
pub trait Subtype
{
    type Parent;

    fn upcast_ref(&self) -> &Self::Parent;
    fn upcast_mut(&mut self) -> &mut Self::Parent;
    fn upcast(self) -> Self::Parent;
}

/// Implements `Subtype` for a type that is owned by an LLVM context.
macro_rules! impl_subtype {
    ($ty:ident => $parent:ident) => {
        impl<'ctx> $crate::subtype::Subtype for $ty<'ctx> {
            type Parent = $parent<'ctx>;

            fn upcast_ref(&self) -> &Self::Parent { &self.0 }
            fn upcast_mut(&mut self) -> &mut Self::Parent { &mut self.0 }
            fn upcast(self) -> Self::Parent { self.0 }
        }

        impl<'a> AsRef<$parent<'a>> for $ty<'a>
        {
            fn as_ref(&self) -> &$parent<'a> { &self.0 }
        }

        impl<'a> ::std::ops::Deref for $ty<'a> {
            type Target = $parent<'a>;

            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl<'a> ::std::ops::DerefMut for $ty<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }
}
