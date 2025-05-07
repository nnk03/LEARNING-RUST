#![allow(dead_code)]

// ?????
// use proc_macro::TokenStream;

// example of declarative macro
#[macro_export]
macro_rules! custom_vec {
    ( $($x : expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
            temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub mod hello_macro {
    pub trait HelloMacro {
        fn hello_macro();
    }
}
