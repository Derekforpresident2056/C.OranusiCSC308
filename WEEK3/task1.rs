fn main() {
    let mut message = String::from("Hello");
    
    show_message(&message);
    add_note(&mut message);
    
    println!("Final message : {}", message);
}

fn show_message(msg: &String) {
    println!("Current message: {}", msg);
}

fn add_note(msg: &mut String) {
    msg.push_str(", world!");
}

//The above snippet would infact run, its just very dependent on the positioning of the functions show_message and add_note. if add note
//were to occur first, the message would be changed entirely. you cannot borrow mutably and immutably from the same reference. My approach is
//to clone the variable 'message' and allow its copy be mutable, this allows the functions to be put in any such order without affecting
//each other at all. 