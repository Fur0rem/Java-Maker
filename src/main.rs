mod higher_order;
mod tokens;
mod translation;

use crate::translation::parser;
use crate::translation::writer;

use std::env;
use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum JavaMakerError {
	#[error("Wrong number of arguments : expected 2, got {0}")]
	WrongNumberOfArguments(usize),
	#[error("Unknown option {0}")]
	UnknownOption(String),
	#[error("Unknown visibility {0}")]
	UnknownVisibility(String),
	#[error("Unknown modifier {0}")]
	UnknownModifier(String),
	#[error("Unknown type {0}")]
	UnknownType(String),
}

fn run_command() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		return Err(Box::new(JavaMakerError::WrongNumberOfArguments(
			args.len() - 1,
		)));
	}

	let command = parser::parse_command(&args[1], Path::new(&args[2]))?;
	writer::create_class(command)?;
	return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let result = run_command();
	if let Err(e) = result {
		println!("Example usage: java_maker \"Person {{ name : String @public (final); age : int @private = 0; height : float @protected = 1.5f }} --getters --docs\" path/to/file.java");
		println!("args: {:?}", env::args());
		return Err(e);
	}

	return Ok(());
}
