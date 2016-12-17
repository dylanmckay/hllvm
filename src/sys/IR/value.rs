pub enum OpaqueValue { }

type ValueRef = *mut OpaqueValue;

extern "C" {
    pub fn LLVMRustIRValueDump(_: ValueRef);
}

#[cfg(test)]
mod test {
    #[test]
    fn dump() {
    }
}
