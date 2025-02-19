use std::io;

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret = 42;
    let mut attempts = 0;

    loop {
        // Simulate user input (you can change the value for testing)
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let guess: i32 = input.trim().parse().unwrap();
        attempts += 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct! The secret number is {}.", secret);
            break;
        } else if result == 1 {
            println!("Your guess of {} is too high.", guess);
        } else {
            println!("Your guess of {} is too low.", guess);
        }
    }

    println!("It took you {} guesses.", attempts);
}