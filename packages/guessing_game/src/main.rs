use rand::Rng;
use std::io;

fn main() {
    let x = rand::thread_rng().gen_range(0..100) + 1;
    println!("Take a guess between 1 and 100: ");
    let mut guess: i32 = read_input();
    while x != guess {
        if x < guess {
            println!("Too high!");
        }
        else {
            println!("Too low!");
        }
        guess = read_input();
    }
    println!("Congratulations! You win!");
}

fn read_input() -> i32 {
    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let guess: i32 = line.trim().parse().expect("Error: input is not a number");
    return guess;
}
