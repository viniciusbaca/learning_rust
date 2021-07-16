use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // Creates a mutable variable that is
    // a new empty instance of a String
    let mut guess = String::new();

    // Returns an instance of std::io::Stdin, a type
    // that represents a handle to terminal input
    io::stdin()
        // Calls a method from the previous instance
        // to append the input into a reference (&)
        // of our mutable variable
        .read_line(&mut guess)
        // The previous method appends the input and
        // returns a io::Result, which if is a Err,
        // the expect method will crash the program
        .expect("Failed to read line.");

    // Curly brackets are placeholders to hold the
    // respective value listed after the String
    println!("You guessed: {}", guess);
}
