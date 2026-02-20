
// Rust Book Chapter 12 Project
// minigrep

use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	// dbg!(&args);

	// let prog_self = &args[0];
	// let query = &args[1];
	// let file_path = &args[2];

	let config = Config::new(&args);

	println!("Searching for `{}` in file `{}`", config.query, config.file_path);

	let contents = fs::read_to_string(config.file_path)
		.expect("Should have been able to read the file");

	println!("With text:\n{contents}");
}

struct Config {
	query: String,
	file_path: String,
}

impl Config {
	fn new(args: &[String]) -> Config {
		let query = args[1].clone();
		let file_path = args[2].clone();

		Config { query, file_path }
	}
}






