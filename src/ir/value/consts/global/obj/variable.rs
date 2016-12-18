use ir::GlobalObject;

/// A function.
pub struct GlobalVariable<'ctx>(GlobalObject<'ctx>);

impl<'ctx> GlobalVariable<'ctx>
{
}

impl_upcast!(GlobalVariable => GlobalObject);
