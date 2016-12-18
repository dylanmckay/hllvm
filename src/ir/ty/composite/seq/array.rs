use ir::SequentialType;

pub struct ArrayType<'ctx>(SequentialType<'ctx>);

impl<'ctx> ArrayType<'ctx>
{
}

impl_upcast!(ArrayType => SequentialType);
