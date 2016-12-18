extern crate llvm;

use llvm::{ir, target};

fn main() {
    let context = ir::Context::new();
    let module = ir::Module::new("mymodule", &context);

    let int8 = ir::IntegerType::new(8, &context);
    let stru = ir::StructType::new(&[&int8.as_ref()], false, &context);

    let func_ty = ir::FunctionType::new(&stru.as_ref(), &[], false);
    let func = module.get_or_insert_function("my_func", &func_ty, &[]);

    let block = ir::Block::new(&context, Some("entry"), Some(&func), None);
    block.append(ir::ReturnInst::new(None, &context).as_ref().as_ref());

    module.dump();
}
