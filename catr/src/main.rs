fn main() {
	let x = catr::get_args().and_then(catr::run);
	
	if let Err(e) = x {
		eprintln!("Some error occured!");
		std::process::exit(1); // exit the program
	}
	
}
