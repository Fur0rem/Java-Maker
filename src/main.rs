mod parser;
mod writer;

mod attribute;
mod visibility;

mod getter_setter;
mod constructor;

mod to_string;
mod equals;

mod comment;

use std::env;


fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		println!("Usage: java_maker \"<command>\"");
		return;
	}

	writer::create_class(parser::parse_command(&args[1]));
}
