
macro_rules! wrap_value {
    ($ffivalue:expr $( => $parent:ident )+ ) => {
        {
            let value = $crate::ir::Value::from_inner($ffivalue);
            wrap_subtype!(value $( => $parent )+)
        }
    }
}

macro_rules! wrap_type {
    ($ffitype:expr $( => $parent:ident )+ ) => {
        {
            let ty = $crate::ir::Type::from_inner($ffitype);
            wrap_subtype!(ty $( => $parent )+)
        }
    }
}


