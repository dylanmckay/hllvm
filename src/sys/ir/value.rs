pub enum OpaqueValue { }

pub type ValueRef = *mut OpaqueValue;

extern "C" {
    pub fn LLVMRustValueDump(_: ValueRef);
}

#[cfg(test)]
mod test {
    #[test]
    fn dump() {
    }
}
