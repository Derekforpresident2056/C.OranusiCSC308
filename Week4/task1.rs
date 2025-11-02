use std::io;

fn last_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate().rev() {
		if item == b' ' {
			return &s[i + 1..];
		}
	}
	&s[ .. ]
}

fn main() {
	let mut my_string = String::new();
	println!("Enter a string please: ");
	io::stdin().read_line(&mut my_string).expect("Failed to read input");
	let word = last_word(&my_string);
	println! ("The last word is: {}", word);
}




