fn main() {
	let x = catr::run();
	
	if let Err(e) = x {
		eprintln!("Some error occured!");
		std::process::exit(1); // exit the program
	}
	
}
