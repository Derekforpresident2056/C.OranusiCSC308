use std::io;

#[derive(Debug)]
struct Bank {
    balance: f32,
    withdrawamount: f32,
    depositamount: f32,
}

impl Bank {
    fn deposit(&self) -> f32 {
        self.balance + self.depositamount
    }

    fn withdraw(&self) -> f32 {
        self.balance - self.withdrawamount
    }

    fn check(&self) {
        println!("Your balance is currently:{}",self.balance);
    }
}

fn main() {
    println!("For the sake of this exercise, we say you have 10k in the bank");
    println!("ENTER THE CORRESPONDING NUMBER TO MAKE SELECT OPTION");
    println!(" ");
    println!("DEPOSIT....................................1");
    println!(" ");
    println!("WITHDRAW...................................2");
    println!(" ");
    println!("CHECK BALANCE..............................3");

    let mut option = String::new();
    io::stdin().read_line(&mut option).expect("Failed to read input");
    let option = option.trim();

    if option == "1"{
        println!("Please enter the amount you wish to deposit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let trimmed = input.trim();
        match trimmed.parse::<f32>() {
            Ok(number) => {
                let bankroll = Bank { 
                    depositamount: number,
                    balance: 10000.0,
                    withdrawamount:0.0,
                 };
                bankroll.check();
                println!("Amount Deposited,NEW Current balance: { :? }", bankroll.deposit());
            }
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }else if option == "2"{
        println!("Please enter the amount you wish to withdraw:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let trimmed = input.trim();
        match trimmed.parse::<f32>() {
            Ok(number) => {
                let bankroll = Bank { 
                    withdrawamount: number,
                    balance: 10000.0,
                    depositamount: 0.0,
                 };
                bankroll.check();
                println!("Amount Withdrawn, Current balance: { :? }", bankroll.withdraw());
            }
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }

    }else if option == "3"{
        let bankroll = Bank{
            balance: 10000.0,
            depositamount: 0.0,
            withdrawamount: 0.0,
        };
        bankroll.check();
    }else{
        println!("ERROR");
    }

}