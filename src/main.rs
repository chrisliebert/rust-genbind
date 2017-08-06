// Copyright(C) 2017 Chris Liebert
extern crate genbind;

pub fn main() {
	use std::env;
	// Default arguments
	let mut input_file = String::from("src/tests/test.rs");
	let mut output_file = String::from("myheader.h");
	
	if env::args().len() == 1 {
		println!("Using default arguments, consider specifying input and output files");
	}
	
	// Check for command-line arguments
	let mut i = 0;
	for argument in env::args() {
        if i == 1 {
            input_file = argument;
        } else if i == 2 {
        	output_file = argument;
        }
        i += 1;
    }
	
	genbind::write_c_header(
		&std::path::Path::new(&input_file),
		&std::path::Path::new(&output_file)
	);
}
