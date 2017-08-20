// Copyright (C) 2017 Chris Liebert
#![crate_type = "lib"]
#![crate_name = "examplelib"]

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

#[repr(C)]
pub struct IntVector3 {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub z: libc::c_int,
}

#[no_mangle]
pub fn add_int_vector3s(a: *const IntVector3, b: *const IntVector3) -> IntVector3 {
    unsafe {
        IntVector3 {
            x: (*a).x + (*b).x,
            y: (*a).y + (*b).y,
            z: (*a).z + (*b).z,
        }
    }
}

#[repr(C)]
pub enum MyChoice {
    OPTION1,
    OPTION2,
    OPTION3,
}


#[no_mangle]
pub fn is_option2(choice: *const MyChoice) -> bool {
    unsafe {
        match *choice {
            MyChoice::OPTION2 => true,
            _ => false,
        }
    }
}

#[no_mangle]
pub fn print_my_choice(choice: *const MyChoice) {
    unsafe {
        match *choice {
            MyChoice::OPTION1 => println!("You picked Option 1"),
            MyChoice::OPTION2 => println!("You picked Option 2"),
            MyChoice::OPTION3 => println!("You picked Option 3"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
