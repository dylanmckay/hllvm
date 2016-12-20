extern crate llvm;

use llvm::{ir, target, support, pass};

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

fn compile_module(module: ir::Module, file_type: target::FileType) {
    let target = target::Registry::get().targets().find(|t| t.name() == "x86-64").expect("doesn't support X86-64");
    let machine = target.create_machine("", "", "");
    let stdout = support::FileOutputStream::stdout(false);
    let pm = pass::Manager::new();

    machine.add_passes_to_emit_file(&pm, stdout.as_ref(), file_type);
    pm.run(module);
    drop(machine);
}

fn main() {
    let context = ir::Context::new();
    let module = build_module(&context);

    module.dump();
    compile_module(module, target::FileType::Assembly);
}
