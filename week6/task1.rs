fn main() {
    let fact = |x: u32| -> u32 {
        let mut result = 1;
        for i in 1..=x {
            result *= i;
        }
        result
    };

    println!("{}", fact(5)); 
}