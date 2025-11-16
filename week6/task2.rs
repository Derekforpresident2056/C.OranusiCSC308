fn main() {
    let numbers: Vec<u32> = (1..=20).collect();

    let is_even = |x: &u32| x % 2 == 0;

    let even_numbers: Vec<u32> = numbers.iter().filter(is_even).cloned().collect();

    println!("Even numbers: {:?}", even_numbers);
}