fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut clonedString = s.clone();
    clonedString.push_str("World!");
    clonedString
}

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    for i in low..=high{
        *total += i;

    }
    
}

fn main() {
    // Problem #1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    // Problem #2
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    // Problem #3
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total);
}