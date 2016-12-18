use ir::SequentialType;

pub struct PointerType<'ctx>(SequentialType<'ctx>);

impl<'ctx> PointerType<'ctx>
{
}

impl_upcast!(PointerType => SequentialType);
