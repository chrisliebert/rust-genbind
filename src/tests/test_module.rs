// Copyright (C) 2017 Chris Liebert

#![allow(dead_code)]
#![allow(private_no_mangle_fns)]
#![allow(unused_variables)]
extern crate libc;

pub struct StructNoReprC {
    data1: bool,
    data2: libc::c_int,
}

pub enum EnumNoReprC {
    OPTION1,
    OPTION2,
    OPTION3,
    OPTION4,
    OPTION5,
}

#[repr(C)]
pub struct StructWithReprC {
    data1: bool,
    data2: libc::c_int,
}

#[repr(C)]
pub enum EnumWithReprC {
    OPTION1,
    OPTION2,
    OPTION3,
    OPTION4,
    OPTION5,
}

#[repr(C)]
pub struct NestedWithReprC {
    amount: libc::c_double,
    nested_struct: StructWithReprC,
    nested_enum: EnumWithReprC,
}

#[repr(C)]
pub struct NestedWithPointerToType {
    const_value_ptr: *const libc::c_float,
    mut_value_ptr: *mut libc::c_float,
    nested_struct_ptr: *mut StructWithReprC,
    nested_enum_ptr: *mut EnumWithReprC,
    nested_struct_ptr_ptr: *mut *mut StructWithReprC,
    nested_struct_ptr_ptr_ptr: *mut *mut *mut StructWithReprC,
}

#[no_mangle]
pub unsafe fn with_unsafe_keyword() {}

#[no_mangle]
pub fn with_int_parameter(number: libc::c_int) {}

#[no_mangle]
pub fn with_const_char_ptr(c_str: *const libc::c_char) {
    use std::ffi::CStr;
    let string: String = unsafe { CStr::from_ptr(c_str).to_string_lossy().into_owned() };
    println!("Hello {}", &string);
}

#[no_mangle]
pub fn with_two_parameters(i: libc::c_int, l: libc::c_long) {
    println!("Hello {:?}, {:?}", i as i32, l as i64);
}

#[no_mangle]
pub fn with_bool_return_type() -> bool {
    true
}

#[no_mangle]
pub fn with_int_return_type() -> libc::c_int {
    53
}

#[no_mangle]
pub fn with_libcc_float_t_parameter(p: libc::c_float) {}

#[no_mangle]
pub fn with_libcc_double_t_parameter(p: libc::c_double) {}

#[no_mangle]
pub fn with_libcc_char_t_parameter(p: libc::c_char) {}

#[no_mangle]
pub fn with_libcc_short_t_parameter(p: libc::c_short) {}

#[no_mangle]
pub fn with_libcc_int_t_parameter(p: libc::c_int) {}

#[no_mangle]
pub fn with_libcc_long_t_parameter(p: libc::c_long) {}

#[no_mangle]
pub fn with_libcc_longlong_t_parameter(p: libc::c_longlong) {}

#[no_mangle]
pub fn with_libcc_schar_t_parameter(p: libc::c_schar) {}

#[no_mangle]
pub fn with_libcc_uchar_t_parameter(p: libc::c_uchar) {}

#[no_mangle]
pub fn with_libcc_ushort_t_parameter(p: libc::c_ushort) {}

#[no_mangle]
pub fn with_libcc_uint_t_parameter(p: libc::c_uint) {}

#[no_mangle]
pub fn with_libcc_ulong_t_parameter(p: libc::c_ulong) {}

#[no_mangle]
pub fn with_libcc_ulonglong_t_parameter(p: libc::c_ulonglong) {}

#[no_mangle]
pub fn with_libcint8_t_parameter(p: libc::int8_t) {}

#[no_mangle]
pub fn with_libcint16_t_parameter(p: libc::int16_t) {}

#[no_mangle]
pub fn with_libcint32_t_parameter(p: libc::int32_t) {}

#[no_mangle]
pub fn with_libcint64_t_parameter(p: libc::int64_t) {}

#[no_mangle]
pub fn with_libcuint8_t_parameter(p: libc::uint8_t) {}

#[no_mangle]
pub fn with_libcuint16_t_parameter(p: libc::uint16_t) {}

#[no_mangle]
pub fn with_libcuint32_t_parameter(p: libc::uint32_t) {}

#[no_mangle]
pub fn with_libcuint64_t_parameter(p: libc::uint64_t) {}

#[no_mangle]
pub fn with_struct_ptr_parameter(
    struct_struct_ptr: *mut StructWithReprC,
    nested_struct_ptr_ptr: *mut *mut StructWithReprC,
) {
}

#[no_mangle]
pub fn new_struct_on_heap() -> std::boxed::Box<StructWithReprC> {
    Box::new(StructWithReprC {
        data1: true,
        data2: 4,
    })
}
