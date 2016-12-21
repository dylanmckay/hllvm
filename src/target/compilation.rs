use target::Target;
use {ir, target, support, pass};

/// A compilation builder.
pub struct CompilationBuilder<'target>
{
    /// The target to build with.
    target: &'target Target,
    /// The target triple.
    triple: target::Triple,
    /// The target CPU.
    cpu: target::CPU,
    /// The current features.
    features: Vec<String>,
}

impl<'target> CompilationBuilder<'target>
{
    /// Creates a new compilation builder.
    pub fn new(target: &'target Target) -> Self {
        CompilationBuilder {
            target: target,
            triple: target::Triple::default(),
            cpu: target::CPU::default(),
            features: Vec::new(),
        }
    }

    /// Sets the target triple.
    pub fn triple(&mut self, triple: target::Triple) -> &mut Self {
        self.triple = triple;
        self
    }

    /// Sets the target CPU.
    pub fn cpu(&mut self, cpu: target::CPU) -> &mut Self {
        self.cpu = cpu;
        self
    }

    /// Adds a feature.
    pub fn add_feature(&mut self, name: &str) -> &mut Self {
        self.features.push(format!("+{}", name));
        self
    }

    /// Removes a feature.
    pub fn remove_feature(&mut self, name: &str) -> &mut Self {
        self.features.push(format!("-{}", name));
        self
    }

    /// Builds the compilation.
    pub fn build(&mut self) -> Compilation {
        let feature_str: String = self.features.join("");

        let machine = self.target.create_machine(&self.triple.0,
                                                 &self.cpu.0,
                                                 &feature_str);
        Compilation { machine: machine }
    }
}

/// A compilation.
pub struct Compilation
{
    machine: target::Machine,
}

impl Compilation
{
    /// Creates a new target using the default configuration.
    ///
    /// Use `CompilationBuilder` for more flexibility.
    pub fn new(target: &Target) -> Self {
        CompilationBuilder::new(target).build()
    }

    /// Compiles a module.
    pub fn compile<S>(&self,
                      module: ir::Module,
                      stream: S,
                      file_type: target::FileType)
        where S: AsRef<support::OutputStream> {
        let mut pm = pass::Manager::new();

        self.machine.add_passes_to_emit_file(&pm, stream.as_ref(), file_type);
        pm.run(module);
    }
}
