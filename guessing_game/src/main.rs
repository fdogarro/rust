use std::io;
use rand::Rng;

fn main() -> io::Result<()>{
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    
    println!("Please input your guess");

    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    // let input = input.trim().parse();

    println!("You typed: {}", input.trim());
    println!("The secret number is: {}", secret_number);

    // if input == secret_number{
    //     println!("You guessed the secret number: {}", secret_number)
    // }else{
    //     println!("Please try again!")
    // }

    Ok(())
}
