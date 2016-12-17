extern crate gcc;

// llvm-config --libs
const LLVM_LIBS: &'static [&'static str] = &[
  "LLVMObjCARCOpts", "LLVMSymbolize", "LLVMDebugInfoPDB",
  "LLVMDebugInfoDWARF", "LLVMGlobalISel", "LLVMCoverage", "LLVMTableGen",
  "LLVMOrcJIT", "LLVMMIRParser", "LLVMNVPTXCodeGen", "LLVMNVPTXDesc", "LLVMNVPTXInfo",
  "LLVMNVPTXAsmPrinter", "LLVMARMDisassembler", "LLVMARMCodeGen", "LLVMARMAsmParser",
  "LLVMARMDesc","LLVMARMInfo", "LLVMARMAsmPrinter", "LLVMAMDGPUDisassembler", "LLVMAMDGPUCodeGen",
  "LLVMAMDGPUAsmParser", "LLVMAMDGPUDesc", "LLVMAMDGPUInfo", "LLVMAMDGPUAsmPrinter",
  "LLVMAMDGPUUtils", "LLVMObjectYAML", "LLVMX86Disassembler", "LLVMX86AsmParser",
  "LLVMX86CodeGen", "LLVMSelectionDAG", "LLVMAsmPrinter", "LLVMDebugInfoCodeView",
  "LLVMX86Desc", "LLVMMCDisassembler", "LLVMX86Info", "LLVMX86AsmPrinter", "LLVMX86Utils",
  "LLVMMCJIT", "LLVMLibDriver", "LLVMOption", "LLVMLineEditor", "LLVMPasses", "LLVMipo",
  "LLVMVectorize", "LLVMLinker", "LLVMIRReader", "LLVMAsmParser", "LLVMInterpreter",
  "LLVMExecutionEngine", "LLVMRuntimeDyld", "LLVMObject", "LLVMMCParser", "LLVMCodeGen",
  "LLVMTarget", "LLVMScalarOpts", "LLVMInstCombine", "LLVMInstrumentation", "LLVMTransformUtils",
  "LLVMMC", "LLVMBitWriter", "LLVMBitReader", "LLVMAnalysis", "LLVMProfileData", "LLVMCore", "LLVMSupport"
];

fn main() {
    gcc::Config::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("lib.cpp")
        .file("ir/value.cpp")
        .file("ir/constants.cpp")
        .file("ir/attributes.cpp")
        .file("ir/function.cpp")
        .file("ir/block.cpp")
        .file("ir/ty.cpp")
        .file("ir/context.cpp")
        .file("ir/module.cpp")
        .compile("libllvm-sys.a");

    println!("cargo:rustc-link-search=native=/usr/local/Cellar/llvm/3.9.0/lib");

    for library in LLVM_LIBS {
        println!("cargo:rustc-link-lib=static={}", library);
    }
}
