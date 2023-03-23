use std::error::Error;
use std::fs::File;
use std::fs;
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
		match read_file(&filename) { // using the open function 
			Err(err) => eprintln!("Failed to read {}: {}", filename, err),
			Ok(contents)	 => println!("contents: {}", contents),	
	} 
}
	Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> { // its a box trait for reading text, it can assign memory as needed at compile time
	match filename { 
		"-" => Ok(Box::new(BufReader::new(io::stdin()))), // reads from stdin if -
 		 _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // opens file if nothing is passed
	}

	//let contents = fs::read_to_string(filename);
	//println!("{}", contents);
}


fn read_file(filename: &str) -> MyResult<String> {
    let mut file = open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
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

