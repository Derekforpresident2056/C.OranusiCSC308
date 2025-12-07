use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::SystemTime;
use std::io::Write;
use std::process;
use std::io;


fn write_to_file() {

    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .unwrap();

    let mut user_input = String::new();
    println!("Please enter the text you want to log:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    writeln!(file,"[{}] {}",timestamp, user_input).unwrap();
}

fn read_file(){
    let file = File::open("notes.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }

}

fn main() {
    println!("SELECT NUMBER THAT CORRELATES WITH OPTION");
    println!("WRITE TO FILE...........................1");
    println!("READ THE FILE...........................2");
    println!("EXIT....................................3");

    let mut option0 = String::new();
    println!("Please enter the Option you have selected:");

    io::stdin()
        .read_line(&mut option0)
        .expect("Failed to read line");

    let option = option0.trim();

    if option == "1" {
        write_to_file();
        main();
    }
    else if option == "2" {
        read_file();
        main();
    }
    else if option == "3" {
        println!("Exiting.....");
        process::exit(1);
    }
    else {
        println!("Invalid selection, please re-enter option");
        main();
    }
}