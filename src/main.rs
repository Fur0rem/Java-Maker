mod parser;
mod writer;

mod attribute;
mod visibility;

mod constructor;
mod getter_setter;

mod equals;
mod to_string;

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
