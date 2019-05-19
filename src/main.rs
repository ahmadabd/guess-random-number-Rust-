use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    // Get a random number between 1 to 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{    // Make a loop 

        println!("Enter a number:");
        let mut guess = String::new();

        // Get a value from user
        io::stdin().read_line(&mut guess)
            .expect("Fail to read");

        // let guess: u32 = guess.trim().parse()
        //    .expect("cannot convert");

        // Convert string to integer
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
        };

        // Make a condition for comparing random number and user input
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal :) YOU WIN");
                break;
            }
        }
    }
}
