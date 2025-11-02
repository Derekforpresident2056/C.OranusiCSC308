use std::io;

fn longest_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	let mut start = 0;
	let mut longest = "";

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			let word = &s[start..i];
			if longest.is_empty() || word.len() > longest.len() {
				longest = word;
			}
			start = i + 1;
		}
	}
	&s[ .. ];

	let word = &s[start..];
	if longest.is_empty() || word.len() > longest.len() {
		longest = word;
	}
	longest
}

fn shortest_word(s: &String) -> &str {
	let bytes = s.as_bytes();
	let mut start = 0;
	let mut shortest = "";

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			let word = &s[start..i];
			if shortest.is_empty() || word.len() < shortest.len() {
				shortest = word;
			}
			start = i + 1;
		}
	}
	&s[ .. ];

	let word = &s[start..];
	if shortest.is_empty() || word.len() < shortest.len() {
		shortest = word;
	}
	shortest
}

fn main() {
	let mut my_string = String::new();
	println!("Enter a string please: ");
	io::stdin().read_line(&mut my_string).expect("Failed to read input");
	let lword = longest_word(&my_string);
	println! ("The longest word is: {}", lword);
	let sword = shortest_word(&my_string);
	println! ("The shortest word is: {}", sword);
}



