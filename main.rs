fn main() {
    println!("hello world");
    println!("\n");
    let cf64 = 50.0;
    let if64 = 1.8;
    let of64 = 32.0;
    let f = ((cf64 * if64) + of64);

    println!("{}° farenheit ", f);
    println!("\n");

    let billf64 = 50000.00;

    if billf64 > 5000.00 {
        println!("ORIGINAL BILL VALUE: {}",billf64);
        println!(" YOU'RE ELIGIBLE FOR A 10 PERCENT DISCOUNT!");
        let new = (0.90 * billf64);
        println!("NEW BILL: {}",new);

    } else if billf64 > 10000.00{
        println!("ORIGINAL BILL VALUE: {}",billf64);
        println!(" YOU'RE ELIGIBLE FOR A 10 PERCENT DISCOUNT!");
        let new = (0.85 * billf64);
        println!("NEW BILL: {}",new);

    }else{
        println!(" YOU'RE NOT ELIGIBLE FOR ANY DISCOUNTS");
        println!("BILL: {}",billf64);
    }
}
