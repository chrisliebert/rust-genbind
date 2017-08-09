// Copyright(C) 2017 Chris Liebert
extern crate genbind;

pub fn main() {
    use std::env;
    // Default arguments
    let mut input_file = String::from("src/tests/test.rs");
    let mut output_file = String::from("myheader.h");

    let args: Vec<String> = env::args().collect();

    // argument 0 is the executable name
    if args.len() != 3 {
        println!("Usage: {} <input_source>.rs <output_header>.h", args[0]);
    }

    // Check for command-line arguments
    let mut i = 0;
    for argument in args {
        if i == 1 {
            input_file = argument;
        } else if i == 2 {
            output_file = argument;
        }
        i += 1;
    }
    println!("Reading source from {}", &input_file);

    genbind::write_c_header(
        &std::path::Path::new(&input_file),
        &std::path::Path::new(&output_file),
    );

    println!("Wrote {}", &output_file);
}
