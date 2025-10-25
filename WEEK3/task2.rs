fn main() {
    let name = String::from("Ada");
    let mut name2:String = name.clone();

    print_name(&name);
    append_title(&mut name2);
    
    println!("Final name: {}", name2);
}

fn print_name(n: &String) {
    println!("Name: {}", n);
}

fn append_title(n: &mut String) {
    n.push_str(" Lovelace");
}
