// Copyright (C) 2017 Chris Liebert

extern crate genbind;

fn main() {
    genbind::write_c_header(
        &std::path::Path::new("lib.rs"),
        &std::path::Path::new("example.h"),
    );
}
