// Copyright (C) 2017 Chris Liebert

// Do not append to this file, additional test declarations can be added to test_module.rs
// which is compiled before running to tests to ensure no errors exist

#[no_mangle]
pub fn simple() {}

// mangled() should not show up since it is not marked with #[no_mangle]
pub fn mangled() {}

mod test_module;
