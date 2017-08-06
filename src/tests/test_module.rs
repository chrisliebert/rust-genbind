// Copyright (C) 2017 Chris Liebert

#![allow(dead_code)]
#![allow(private_no_mangle_fns)] 
#![allow(unused_variables)]
extern crate libc;

#[no_mangle]
pub unsafe fn with_unsafe_keyword() {}

#[no_mangle]
pub fn with_int_parameter(number: libc::c_int) {}

#[no_mangle]
pub fn with_const_char_ptr(c_str: *const libc::c_char) {
	use std::ffi::CStr;
	let string: String = unsafe{ CStr::from_ptr(c_str).to_string_lossy().into_owned() };
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
