// Copyright (C) 2017 Chris Liebert
extern crate libc;

#[no_mangle]
pub fn print_hello() {
    println!("hello");
}

#[no_mangle]
pub fn print_cstr(c_str: *const libc::c_char) {
    use std::ffi::CStr;
    let rust_string: String = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
    println!("{}", &rust_string);
}

#[no_mangle]
pub fn print_int(i: libc::c_int) {
    let rust_int: i32 = i as i32;
    println!("{:?}", rust_int);
}

#[no_mangle]
pub fn print_double(d: libc::c_double) {
    let rust_double: f64 = d as f64;
    println!("{:?}", rust_double);
}

#[no_mangle]
pub fn add_doubles(a: libc::c_double, b: libc::c_double) -> libc::c_double {
    a + b
}

#[no_mangle]
pub fn add_ints(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
