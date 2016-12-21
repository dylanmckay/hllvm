extern crate hllvm;

use hllvm::{ir, target, support};

fn build_module(context: &ir::Context) -> ir::Module {
    let module = ir::Module::new("mymodule", context);

    let int8 = ir::IntegerType::new(8, &context);
    let stru = ir::StructType::new(&[&int8.as_ref()], false, context);

    let func_ty = ir::FunctionType::new(&stru.as_ref(), &[], false);

    {
        let func = module.get_or_insert_function("my_func", &func_ty, &[]);

        let block = ir::Block::new(&context, Some("entry"), Some(&func), None);
        block.append(ir::ReturnInst::new(None, context).as_ref().as_ref());
    }

    module
}

fn main() {
    let context = ir::Context::new();
    let module = build_module(&context);

    module.dump();
    let stdout = support::FileOutputStream::stdout(false);

    let target = target::Registry::get().targets().find(|t| t.name() == "x86-64").expect("doesn't support X86-64");
    target::Compilation::new(&target)
        .compile(module, &stdout, target::FileType::Assembly);
}
