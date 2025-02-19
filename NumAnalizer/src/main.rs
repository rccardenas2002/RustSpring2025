fn even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [4, 12, 3, 6, 9, 21, 22, 2, 13, 26];
    
    
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            let either = if even(num) { "even" } else { "odd" };
            println!("{}: {}", num, either);
        }
    }
    
    
    let mut i = 0;
    let mut sum = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("Sum of all numbers: {}", sum);
    
    
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}

