use std::io;
// Use the std::io library which allows reading from stdin

fn main() {
	let mut inpt = String::new();
	// Create a mutable variable called inpt
	// that is a new instance of a String.

	println!("Input some text: ");
	io::stdin()
		.read_line(&mut inpt)
		.expect("Failed to read line");
	println!("Read: '{}' from stdin", inpt);
}
