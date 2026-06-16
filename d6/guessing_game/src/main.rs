use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    println!("Hello Abhi");
    println!("Enter  the number");

    let mut guess : String = String :: new();

    let secret_number : u32 = rand::thread_rng().gen_range( 1, 101);

    
    println!("The Secret number is : {}", secret_number);
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guess : u32 = guess.trim().parse().expect("Please type a number");
    println!("Use guessed {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed a smaller number"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You goot it"),
    }
}