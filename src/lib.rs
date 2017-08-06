// Copyright (C) 2017 Chris Liebert

extern crate syntex_syntax as syntax;

use std::ops::AddAssign;
use std::rc::Rc;

use syntax::codemap::{CodeMap, FilePathMapping};
use syntax::errors::{Handler};
use syntax::errors::emitter::{ColorConfig};
use syntax::parse::{self, ParseSess};
use syntax::ast::{Crate, FunctionRetTy, Ident, Item, ItemKind, Mutability, Path, PatKind, TyKind, VariantData};
use syntax::symbol::InternedString;

// TODO: add option to limit output based on selected features.
// TODO: correctly support pointers and Boxed repr(C) structs

fn item_is_nonmangled(item: &Item) -> bool {
	// Return false if item node is not a function
	match &item.node {
		&ItemKind::Fn(_, _, _, _, _, _) => (),
		_ => return false,
	};
	
	// Check for no_mangle segment in item's attributes
	for attribute in &item.attrs {
		let path: &Path = &attribute.path;
		for segment in &path.segments {
			let id: Ident = segment.identifier;
			let attr_str: InternedString = id.name.as_str();
			if attr_str == "no_mangle" {
				return true;
			}
		}
	}
	
	false
}

fn item_is_reprc(item: &Item) -> bool {
	match &item.node {
		&ItemKind::Enum(_, _) => (),
		&ItemKind::Struct(_, _) => (),
		_ => return false
	};
	
	// TODO: determine if pub keyword is also required
	
	// Check for repr(c) segment in item's attributes
	for attribute in &item.attrs {
		let path: &Path = &attribute.path;
		for segment in &path.segments {
			let id: Ident = segment.identifier;
			let attr_str: InternedString = id.name.as_str();
			if attr_str == "repr" {
				return true;
			}
		}
	}
	
	false
}

fn is_c_type(type_str: &str) -> bool {
	match type_str {
		"void" => true,
		"bool" => true,
		"char" => true,
		"short" => true,
		"int" => true,
		"long" => true,
		"float" => true,
		"double" => true,
		"signed char" => true,
		"signed short" => true,
		"unsigned char" => true,
		"unsigned short" => true,
		"unsigned int" => true,
		"unsigned long" => true,
		"unsigned long long" => true,
		_ => false,
	}
}

fn get_equivalent_c_type_string(rust_type: &str) -> String {
	if is_c_type(&rust_type) {
		return String::from(rust_type);
	}
	
	String::from(match rust_type {
		"libcc_void" => "void",
		"libcc_char" => "char",
		"libcc_short" => "short",
		"libcc_int" => "int",
		"libcc_float" => "float",
		"libcc_double" => "double",
		"libcc_long" => "long",
		"libcc_longlong" => "long long",
		"libcc_schar" => "char",
		"libcc_uchar" => "unsigned char",
		"libcc_uint" => "unsigned int",
		"libcc_ulong" => "unsigned long",
		"libcc_ulonglong" => "unsigned long long",
		"libcc_ushort" => "unsigned short",
		"libcint8_t" => "char",
		"libcint16_t" => "short",
		"libcint32_t" => "int",
		"libcint64_t" => "long",
		"libcuint8_t" => "unsigned char",
		"libcuint16_t" => "unsigned short",
		"libcuint32_t" => "unsigned int",
		"libcuint64_t" => "unsigned long",
		
		/*   TODO: support these types from lib.rs in libc crate
		pub type size_t = usize;
		ptrdiff_t = isize;
		intptr_t = isize;
		uintptr_t = usize;
		ssize_t = isize;
		*/
		
		_ => {
			// TODO: search for structs marked with #repr(C) and enable as type param only if boxed, then issue c declaration with pointer to #repr(c)'ed type. Also must convert repr(c) type to C struct and declare initially before parsing function declarations.
			println!("Could not find C equivalent for {}, using void*", rust_type);
			"void*"
		}
	})
}

fn get_type_string(node: &TyKind) -> String {
	let mut type_str: String = String::new();
	match node.clone() {
		TyKind::Path(_q, p) => {
			let mut type_segment_count = p.segments.len();
			let mut the_type_string = String::new();
			for type_segment in &p.segments {
				type_segment_count += 1;
				the_type_string.add_assign(&format!("{}", type_segment.identifier.name.as_str()));
				
				// Add space if there are multiple segments in the type name
				if type_segment_count < p.segments.len() {
					the_type_string.add_assign(" ");
				}
			}
			type_str.add_assign(&get_equivalent_c_type_string(&the_type_string));
		},
		TyKind::Ptr(mut_ty) => {
			// Check to see if variable is const
			match mut_ty.mutbl {
				Mutability::Immutable => type_str.add_assign("const "),	
				Mutability::Mutable => (),	// variables are mutable by default in C
			};
			type_str.add_assign(&get_equivalent_c_type_string(&get_type_string(&mut_ty.ty.unwrap().node)));
			type_str.add_assign("*");
		}
		// TODO: investigate whether arrays can be supported: 
		// TyKind::Array, ...
		_ => {
			//type_str.add_assign(&format!("Unmatch type: {:?}", node));
			println!("Unmatch type: {:?}", node);
			type_str.add_assign("void*");
		},
	}
	return type_str;	
}

pub fn get_c_declarations(input_src: &std::path::Path) -> Vec<String> {
	if !input_src.exists() {
		panic!("{} does not exist", input_src.display());
	}
	let mut c_declarations: Vec<String> = Vec::new();
	let path_file_mapping: FilePathMapping = FilePathMapping::empty();
    let codemap = Rc::new(CodeMap::new(path_file_mapping));
    let can_emit_warnings = true;
    let treat_err_as_bug = false;
    let tty_handler = Handler::with_tty_emitter(ColorConfig::Auto, can_emit_warnings, treat_err_as_bug, Some(codemap.clone()));
    let parse_session = ParseSess::with_span_handler(tty_handler, codemap.clone());
    let result: Crate = parse::parse_crate_from_file(&input_src, &parse_session).expect(&format!("Unable to parse {:?}", input_src));
    let mut all_items: Vec<Item> = Vec::new();
    for item in &result.module.items {
    	match item.node.clone() {
    		ItemKind::Mod(m) => {
    			// Look for modules, and add their items to the search
    			//println!("searching module named {:?}", item.ident.name.as_str());   			
    			for module_item in m.items {
    				all_items.push(module_item.unwrap());
    			}
    		},
    		_ => ()
    	}
    	all_items.push(item.clone().unwrap());
    }
    
    for item in all_items {
    	if item_is_nonmangled(&item) {
    		match item.node {
    			ItemKind::Fn(fndecl, _unsafety, _constness, _abi, _generics, _block) => {
	    			// print return type
	    			let mut c_declaration = String::from("extern ");
	    			match fndecl.output.clone() {
	    				FunctionRetTy::Default(_) => c_declaration.add_assign("void "),
	    				FunctionRetTy::Ty(p) => {
	    					c_declaration.add_assign(&format!("{} ", &get_type_string(&p.unwrap().node)));
	    				}
	    			};
	    			c_declaration.add_assign(&format!("{}(", item.ident.name.as_str()));		
	    			let mut param_count: usize = 0;
	    			for input in &fndecl.inputs {
	    				param_count += 1;
	    				match input.pat.node.clone() {
	    					PatKind::Ident(_binding_mode, spanned_indent, _path) => {
	    						// Parameter type
	    						let rust_type_string = &get_type_string(&input.clone().ty.unwrap().node);
	    						c_declaration.add_assign(&format!("{}", &rust_type_string));
	    						c_declaration.add_assign(&format!(" {}", spanned_indent.node.name.as_str()));
	    					},
	    					PatKind::Struct(_, _, _) => (),
	    					PatKind::Box(_) => (),
	    					PatKind::Ref(_, _mutability) => (),
	    					PatKind::Lit(_) => (),
	    					_ => (),
	    				};
	    				// Put a space between function parameters
	    				if param_count < fndecl.inputs.len() {
	    					c_declaration.add_assign(", ");
	    				}
	    			}
	    			c_declaration.add_assign(");");
	    			c_declarations.push(c_declaration);
    			},
    			// Ignore anything that isn't a function or struct
    			_ => (),
    		}
    	} else if item_is_reprc(&item) {
    		// TODO: add enums and structs to the list of types detected by is_c_type() and get_equivalent_c_type_string() at runtime
    		match item.node {
    			ItemKind::Enum(enum_def, _generics) => {
    				let mut c_enum_declaration: String = String::from("typedef enum ");
    				c_enum_declaration.add_assign(&format!("{} {{ ", item.ident.name.as_str()));
    				let mut variant_count = 0;
    				for variant in &enum_def.variants {
    					variant_count += 1;
    					c_enum_declaration.add_assign(&format!("{}", variant.node.name.name.as_str()));
    					if variant_count < enum_def.variants.len() {
    						c_enum_declaration.add_assign(", ");
    					}
    				}
    				c_enum_declaration.add_assign(&format!(" }} {};", item.ident.name.as_str()));
    				c_declarations.push(c_enum_declaration);
    			},
    			ItemKind::Struct(variant_data, _generics) => {
	    			match variant_data.clone() {
	    				VariantData::Struct(fields, _id) => {
	    					let mut c_struct_delcaration: String = String::from("typedef struct ");
	    					c_struct_delcaration.add_assign(&format!("{} {{ ", item.ident.name.as_str()));
	    					for field in &fields {
		    					match field.ident {
		    						Some(ident) => {
		    							c_struct_delcaration.add_assign(&format!("{} ", &get_equivalent_c_type_string(&get_type_string(&field.clone().ty.unwrap().node))));
		    							c_struct_delcaration.add_assign(&format!("{}; ", ident.name.as_str()));    							
		    						},
		    						None => (),
		    					}
	    					}
	    					c_struct_delcaration.add_assign(&format!("}} {};", item.ident.name.as_str()));
	    					c_declarations.push(c_struct_delcaration);
	    				},
	    				_ => (),
	    			};
    			},
    			_ => (),
    		}
    	}
    }
	c_declarations
}

pub fn write_c_header(input_path: &std::path::Path, output_path: &std::path::Path) {
	use std::error::Error;
	use std::fs::File;
	use std::io::Write;
		
	let mut out_file: File = match File::create(&output_path) {
		Err(e) => panic!("Could not open {} for writing: {}", output_path.display(), e.description()),
		Ok(f) => f,
	};
	
	for c_declaration in get_c_declarations(input_path) {
		match out_file.write(&format!("{}\n", &c_declaration).as_bytes()) {
			Err(e) => panic!("Could not write {} to {}: {}", c_declaration, output_path.display(), e.description()),
			Ok(_) => (),
		}
	}
}

#[cfg(test)]
mod tests {
	use get_c_declarations;
	use std::path::Path;
	
	// The test_module is declared here to ensure the contents of target_module.rs compiles.
	// test.rs can not be included here since it declares test_module
	mod test_module;
    
    #[test]
    fn it_works() {
    	// Since target.rs includes target_module.rs, both will be scanned
		let declarations: Vec<String> = get_c_declarations(Path::new("src/tests/test.rs"));
		// Ensure code in top-level test.rs file is scanned
		assert!(declarations.contains(&String::from("extern void simple();")));
		// Make sure only non-mangled functions are reported
		assert!(!declarations.contains(&String::from("extern void mangled();")));

		// Remaining assertions are regarding the output of processing test_module.rs

		// make sure non repr(C) structures are processed 
		assert!(!declarations.contains(&String::from("typedef struct StructNoReprC { bool data1; int data2; } StructWithReprC;")));
		assert!(!declarations.contains(&String::from("typedef enum EnumNoReprC { OPTION1, OPTION2, OPTION3, OPTION4, OPTION5 } EnumWithReprC;")));

		// check structs and enums with #repr(C)
		assert!(declarations.contains(&String::from("typedef struct StructWithReprC { bool data1; int data2; } StructWithReprC;")));
		assert!(declarations.contains(&String::from("typedef enum EnumWithReprC { OPTION1, OPTION2, OPTION3, OPTION4, OPTION5 } EnumWithReprC;")));
		
		// check function declarations
		assert!(declarations.contains(&String::from("extern void with_unsafe_keyword();")));
		assert!(declarations.contains(&String::from("extern void with_int_parameter(int number);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_schar_t_parameter(char p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_float_t_parameter(float p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_double_t_parameter(double p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_char_t_parameter(char p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_short_t_parameter(short p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_int_t_parameter(int p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_long_t_parameter(long p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_longlong_t_parameter(long long p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_uchar_t_parameter(unsigned char p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_ushort_t_parameter(unsigned short p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_uint_t_parameter(unsigned int p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_ulong_t_parameter(unsigned long p);")));
		assert!(declarations.contains(&String::from("extern void with_libcc_ulonglong_t_parameter(unsigned long long p);")));
		assert!(declarations.contains(&String::from("extern void with_libcint8_t_parameter(char p);")));
		assert!(declarations.contains(&String::from("extern void with_libcint16_t_parameter(short p);")));
		assert!(declarations.contains(&String::from("extern void with_libcint32_t_parameter(int p);")));
		assert!(declarations.contains(&String::from("extern void with_libcint64_t_parameter(long p);")));
		assert!(declarations.contains(&String::from("extern void with_libcuint8_t_parameter(unsigned char p);")));
		assert!(declarations.contains(&String::from("extern void with_libcuint16_t_parameter(unsigned short p);")));
		assert!(declarations.contains(&String::from("extern void with_libcuint32_t_parameter(unsigned int p);")));
		assert!(declarations.contains(&String::from("extern void with_libcuint64_t_parameter(unsigned long p);")));
		assert!(declarations.contains(&String::from("extern void with_const_char_ptr(const char* c_str);")));
		assert!(declarations.contains(&String::from("extern void with_two_parameters(int i, long l);")));
		assert!(declarations.contains(&String::from("extern bool with_bool_return_type();")));
		assert!(declarations.contains(&String::from("extern int with_int_return_type();")));	
	}
}
