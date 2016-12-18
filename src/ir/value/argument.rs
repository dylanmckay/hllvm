use ir::Value;

/// A function argument
pub struct Argument<'ctx>(Value<'ctx>);

impl<'ctx> Argument<'ctx> { }

impl_upcast!(Argument => Value);
