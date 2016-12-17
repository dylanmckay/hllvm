pub enum ValueRef { }

extern {
    pub fn LLVMRustIRValueDump(_: *const ValueRef);
}

#[cfg(test)]
mod test {
    #[test]
    fn dump() {
    }
}
