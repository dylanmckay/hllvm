use ir::Value;

/// A piece of inline assembly.
pub struct InlineAsm<'ctx>(Value<'ctx>);

impl<'ctx> InlineAsm<'ctx> { }

impl_upcast!(InlineAsm => Value);
