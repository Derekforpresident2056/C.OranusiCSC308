fn main() {
    let s1 = String::from("Hi");
    let s2 = String::from("amazing!");
    
    let result1 = longest1(&s1, &s2);
    let result2 = longest2(&s1, &s2);
    println!("The longer string is of length: {}", result1);
    println!("This longer string is: {}", result2);
}

fn longest1(a: &String, b: &String) -> usize {
    let alen = a.len();
    let blen = b.len();

    println!("The 'a' string is: {}", alen);
    println!("The 'b' string is: {}", blen);

    if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    }
}

fn longest2<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
