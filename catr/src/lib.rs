use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>; // custom MyResult type for error handling

#[derive(Debug)]
pub struct Config { // Config struct

	files: Vec<String>,
	number_lines: bool,
	number_nonblank_lines: bool,
}
 
pub fn run(config: Config) -> MyResult<()> {
        for filename in config.files {
		println!("{}",filename);	
	} 
	Ok(())
}


pub fn get_args() -> MyResult<Config> { // returns Config struct if no error
	let matches = App::new("catr")
		.version("0.1")
		.author("decimal <decimal@udev.lol>")
		.about("rust cat")
		.usage("catr -bl <file>")
		.arg(
			Arg::with_name("Files")
			.value_name("FILE")
			.help("file name")
			.required(true)
			.min_values(1)
		)
		.arg(
			Arg::with_name("number_lines")
			.long("nlines")
			.short("n")
			.takes_value(false)
			.conflicts_with("number_non_blank")
			.help("number lines")
		)
		.arg(
			Arg::with_name("number_non_blank")
			.long("blank")
			.short("b")
			.help("non blank lines")
			.takes_value(false)
		)
		.get_matches();
		

	Ok(Config {
		files: matches.values_of_lossy("Files").unwrap(), // honestly, i don't know what this does
		number_lines: matches.is_present("number_lines"), // checks if "number_lines" is there in command line and returns bool
		number_nonblank_lines: matches.is_present("number_non_blank"), // same as above
	}
	)
	
}

