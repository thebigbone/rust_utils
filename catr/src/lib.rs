use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
	files: Vec<String>,
	number_lines: bool,
	number_nonblank_lines: bool,
}
 
pub fn run(config: Config) -> MyResult<()> {
         println!("{:?}",config);
         Ok(())
}


pub fn get_args() -> MyResult<Config> {
	let matches = App::new("catr")
		.version("0.1")
		.author("decimal <decimal@udev.lol>")
		.about("rust cat")
		.usage("catr -bl <file>")
		.arg(
			Arg::with_name("Files")
			.value_name("TEXT")
			.help("file path")
			.required(true)
			.min_values(1)
		)
		.arg(
			Arg::with_name("number lines")
			.long("nlines")
			.short("n")
			.takes_value(false)
			.help("number lines")
		)
		.arg(
			Arg::with_name("number non blank")
			.long("blank")
			.short("b")
			.help("non blank lines")
			.takes_value(false)
		)
		.get_matches();
		
	let files = Vec::new();

	Ok(Config {
		files: files,
		number_lines: true,
		number_nonblank_lines: true,
	}
	)
	

}

