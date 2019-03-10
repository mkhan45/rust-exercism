use std::io;
use fib::*;

fn main() {
    loop{
        println!("Input:");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: usize = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Multiplication table: {}", table_neatprint(multip_table(input)));
        println!("Fib: {}", fib(input));
    }

}
