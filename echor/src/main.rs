use clap::{App,Arg};

fn main() {
    let matches = App::new("echors") // using _ is an indication of not usig the variable for now

	.version("0.2")
	.author("thebigbone <decimal@udev.lol>")
	.about("Rust echo")
	.arg(
		Arg::with_name("text")
		.value_name("TEXT")
		.help("helppp")
		.required(true)
		.min_values(1),
	)
	.arg(
		Arg::with_name("omit_newline")
		.short("n")
		.help("do not print newline")
		.takes_value(false),
	)
	.get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");


    let mut ending = "\n";
    if omit_newline {
	ending = "";
    }

    print!("{}{}", text.join(" "), ending);
}
