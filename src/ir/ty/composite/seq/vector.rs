use ir::SequentialType;

pub struct VectorType<'ctx>(SequentialType<'ctx>);

impl<'ctx> VectorType<'ctx>
{
}

impl_upcast!(VectorType => SequentialType);
