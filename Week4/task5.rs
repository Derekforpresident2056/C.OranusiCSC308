use std::io;
#[derive(Debug)]

struct Student {
    score: i32,
    name: String,
}

impl Student{
	fn check(&self){
		if (self.score >= 40 && self.score < 101){
			println!("{} has passed",self.name );
		}
		else if(self.score <= 40 && self.score > -1){
			println!("{} has failed",self.name );
		}
		else{
			println!("ERROR, invalid entry");
		}
	}
}

fn main(){
	println!("ENTER STUDENT NAME:");
	let mut student = String::new();
	io::stdin().read_line(&mut student).expect("Failed to read input");
	let student = student.trim();
	println!("ENTER STUDENT SCORE:");
	let mut score = String::new();
	io::stdin().read_line(&mut score).expect("Failed to read input");
	let score = score.trim();
	match score.parse::<i32>() {
		Ok(number) => {
			let nameandscore = Student {
				score: number,
				name: student.to_string(),
			};
			nameandscore.check();
		}
		Err(_) => todo!()
	}
}