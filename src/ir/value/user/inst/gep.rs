use SafeWrapper;
use ir::{User, Instruction, Value, Type};
use sys;

/// An instruction which can read elements from a pointer.
pub struct GetElementPtrInst<'ctx>(Instruction<'ctx>);

impl<'ctx> GetElementPtrInst<'ctx>
{
    /// Creates a new GEP instruction.
    pub fn new(pointee_ty: &Type,
               pointer: &Value,
               indices: &[&Value]) -> Self {
        GetElementPtrInst::new_generic(pointee_ty, pointer, indices, false)
    }

    /// Creates a new in-bounds GEP instruction.
    pub fn inbounds(pointee_ty: &Type,
                    pointer: &Value,
                    indices: &[&Value]) -> Self {
        GetElementPtrInst::new_generic(pointee_ty, pointer, indices, true)
    }

    /// Creates a new GEP instruction.
    pub fn new_generic(pointee_ty: &Type,
                       pointer: &Value,
                       indices: &[&Value],
                       in_bounds: bool) -> Self {
        let indices: Vec<_> = indices.iter().map(|i| i.inner()).collect();

        unsafe {
            let inner = sys::LLVMRustCreateGetElementPtrInst(pointee_ty.inner(), pointer.inner(),
                &indices, in_bounds);
            GetElementPtrInst(Instruction(User(Value::from_inner(inner))))
        }
    }
}

impl_subtype!(GetElementPtrInst => Instruction);
