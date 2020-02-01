#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use libc::c_int;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn my_cpp_func_safe(num: c_int) -> c_int {
    let mut cpp_num = 0;
    unsafe {
        cpp_num = my_cpp_func(num);
    }
    cpp_num
}

// Output a function for usage in C.
#[no_mangle]
pub extern "C" fn my_rust_func(num: c_int) -> c_int {
    println!("Rust Number is {}", num);
    let rust_num = num + 1;
    let cpp_num = my_cpp_func_safe(rust_num);
    cpp_num + 1
}
