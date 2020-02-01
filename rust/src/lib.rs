extern crate libc;

use libc::c_int;
use std;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}

// Output a function for usage in C.
#[no_mangle]
pub extern "C" fn my_exported_func(num: c_int) -> c_int {
    println!("Rust Number is {}", num);
    num + 1
}
