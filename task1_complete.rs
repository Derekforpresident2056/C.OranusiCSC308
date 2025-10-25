fn main() {
    let message = String::from("Hello");
    let mut message2:String = message.clone();
    
    add_note(&mut message2);
    show_message(&message);
    
    println!("Final message: {}", message2);
}

fn show_message(msg: &String) {
    println!("Current message: {}", msg);
}

fn add_note(msg: &mut String) {
    msg.push_str(", world!");
}

//Here, by cloning the variable 'message', I've been able to achieve the mutable and immutable borrowing by cloning the variable.