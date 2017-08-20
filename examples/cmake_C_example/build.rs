// Copyright (C) 2017 Chris Liebert

extern crate cmake;
extern crate genbind;

use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[cfg(debug_assertions)]
fn is_debug() -> bool {
    true
}

#[cfg(not(debug_assertions))]
fn is_debug() -> bool {
    false
}

/// Return the created time of file if OS supports,
/// otherwise returns now()
fn get_file_time(path: &Path) -> SystemTime {
    match path.metadata() {
        Ok(metadata) => {
            match metadata.created() {
                Ok(time) => time,
                Err(_) => SystemTime::now(),
            }
        },
        Err(_) => {
            SystemTime::now()
        }
     } 
}

fn find_dependent_shared_library(filename_prefix: &str, filename_suffix: &str) -> String {
    let build_type: String = match is_debug() {
        true => String::from("debug"),
        false => String::from("release"),
    };

    let mut possible_matches: Vec<(String, SystemTime)> = Vec::new();
    let paths = fs::read_dir(&format!("target/{}/deps", &build_type))
        .expect("Unable to open target buld directory");
    for path in paths {
        let result: std::result::Result<std::fs::DirEntry, std::io::Error> = path;
        match result {
            Ok(r) => {
                let possible: String = format!("{}", r.path().display());
                let possible_copy: String = possible.clone();
                let possible_path: &Path = Path::new(&possible_copy);
                match possible_path.file_name() {
                    Some(filename_osstr) => {
                        match filename_osstr.to_str() {
                            Some(filename_str) => {
                                if filename_str.starts_with(filename_prefix) &&
                                    filename_str.ends_with(filename_suffix)
                                {
                                    possible_matches.push((
                                        possible,
                                        get_file_time(&possible_path)
                                    ));
                                }
                            }
                            None => (),
                        }
                    }
                    None => (),
                };
            }
            Err(_) => (),
        };
    }

    let mut best: Option<(String, SystemTime)> = None;
    for possible in possible_matches {
        match best.clone() {
            Some(tuple) => {
                let tuple: (String, SystemTime) = tuple;
                let best_time: SystemTime = tuple.1.clone();
                if best_time < possible.1 {
                    best = Some(possible);
                }
            }
            None => {
                best = Some(possible);
            }
        }
    }

    match best {
        Some(b) => b.0,
        None => panic!("Unable to find dependent shared library"),
    }
}

#[cfg(target_os = "windows")]
fn copy_shared_library() -> std::io::Result<()> {
    let path_str: String = find_dependent_shared_library("examplelib", ".lib");
    try!(fs::copy(Path::new(&path_str), Path::new("examplelib.lib")));
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn copy_shared_library() -> std::io::Result<()> {
    let path_str: String = find_dependent_shared_library("libexamplelib", ".a");
    try!(fs::copy(Path::new(&path_str), Path::new("libexamplelib.a")));
    Ok(())
}

fn main() {
    copy_shared_library().expect("Unable to copy shared library");

    genbind::write_c_header(
        &std::path::Path::new("../library/lib.rs"),
        &std::path::Path::new("example.h"),
    );

    // cmake build
    let dst = cmake::Config::new(".").build();
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-search=native={}", dst.display());
}
