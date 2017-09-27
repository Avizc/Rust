extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");
    let secret_number=rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}",secret_number);
    loop{
        println!("Please input your guess.");
        let mut guess=String::new();
        io::stdin().read_line(&mut guess) // associated function `stdin` on `io`
            .expect("Failed to read line");
        // Former guess illed the program, new guess continues on after prompting the user to enter an int
        // let guess:u32=guess.trim().parse() // Rust allows you to shadow prev val of guess with a new one
            // .expect("Please type a number!");
        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        println!("You guessed: {}",guess);
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("Too small!"),
            Ordering::Greater=>println!("Too big!"),
            Ordering::Equal=>{
                println!("You win!");
                break;
            }
        }
    }
}

// Avi's notes!
// `let` is immutable
// `mut` allows it to become mutable!
// ::new() indicates new is being an associated function of said type (String in this case) on that type instead of a specific case of it
// `read_line` takes user input and converts to a string
// `&` is indicating the argument is a reference
// references are also immutable by default