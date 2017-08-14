// Copyright (C) 2017 Chris Liebert

extern crate examplelib;
extern crate libc;

#[cfg(test)]
mod tests {
    use libc;

    #[test]
    fn print_functions() {
        use examplelib::{print_cstr, print_double, print_int, print_hello};
        use std::ffi::CString;
        let cstring: CString = CString::new("This is a test C string".as_bytes()).unwrap();
        let cstr: *const i8 = cstring.as_ptr();
        print_cstr(cstr);
        print_double(12.54);
        print_int(64);
        print_hello();
    }

    #[test]
    fn add_ints() {
        use examplelib::add_ints;
        let expected: libc::c_int = 7;
        let result: libc::c_int = add_ints(3, 4);
        assert!(result == expected);
    }

    #[test]
    fn add_doubles() {
        use examplelib::add_doubles;
        let expected: libc::c_double = 10.3;
        let result: libc::c_double = add_doubles(2.8, 7.5);
        assert!(result == expected);
    }
}
