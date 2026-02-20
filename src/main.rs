
// Rust Book Chapter 12 Project
// minigrep

use std::{
	env,
	fs,
	process,
	error::Error
};

fn main() {
	let args: Vec<String> = env::args().collect();
	// dbg!(&args);

	// let prog_self = &args[0];
	// let query = &args[1];
	// let file_path = &args[2];

	// let config = Config::new(&args);

	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem parsing arguments: {err}");
		process::exit(1);
	});

	println!("Searching for `{}` in file `{}`", config.query, config.file_path);

	if let Err(e) = run(config) {
		println!("Application error: {e}");
		process::exit(1);
	}
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file_path)?;
	// .expect("Should have been able to read the file");

	println!("With text:\n{contents}");

	Ok(())
}

struct Config {
	query: String,
	file_path: String,
}



impl Config {
	/*
	fn new(args: &[String]) -> Config {
		if args.len() < 3 {
			panic!("not enough arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Config { query, file_path }
	}
	*/

	fn build(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}

		let query = args[1].clone();
		let file_path = args[2].clone();

		Ok(Config { query, file_path })
	}
}






