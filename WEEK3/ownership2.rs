fn take_ownership(name:String){ //here "name" is the de facto variable for this function. 
	println!("Hello {}",name);
}
//name is valid up until the end of the function, memory is freed and the value/content of the memory
//location is not returned to main branch

fn take_ownership_and_give_back(name2: String) -> String {
	//here "name2" is the de facto variable for this function. 
	println!("Hello {}",name2);
	let name2: String = String::from("Operating Systems(edited)");
	let addition = " concatenated";
	let name2 = name2 + addition;
	name2 //name has been returned to main branch
}


fn main(){
	let s: String = String::from("CSC 308"); // s comes into scope, s has been defined
	take_ownership(s);
	//here we've passed the value of the string object into the function, It is no longer useable by the main branch.

	let g: String = String::from("Operating Systems"); // g comes into scope, g has been defined
	let g_returned = take_ownership_and_give_back(g);
	println!("If this works, then {} has been returned to main branch",g_returned);


}