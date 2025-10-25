fn calculate_length_ref(s: &String) -> usize {
	let lengthof = s.len();
	println!("length of string is {}",lengthof );
	s.len()
}

fn main(){
	let name:String= String::from("abcdefghijklmnopqrstuvwxyz");
	let length: usize = calculate_length_ref(&name);
	println!("The length of the name '{}' is {}",name,length);
}