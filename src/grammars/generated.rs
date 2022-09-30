macro_rules! include_generated {
    ($name:ident) => {
        include!(concat!(
            env!("OUT_DIR"),
            "/generated/",
            stringify!($name),
            ".mod"
        ));
    };
}

include_generated!(macrointconstantexpressionast);
