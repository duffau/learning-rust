use std::io;

fn main() {
	let mut my_string = String::new();
	println!("Input string to be reversed:");
	io::stdin().read_line(&mut my_string);
	let my_string = my_string.trim();
	let mut my_reversed_string = String::new();
	for c in my_string.chars().rev() {
		my_reversed_string += &c.to_string();
	} 
	println!("Rerversed: {}", my_reversed_string);
} 	