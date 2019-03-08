fn main() {
    println!("{}", is_palindrome("racecar"));
}

fn is_palindrome(input: &str) -> bool{
    let comp_string: String = input.to_lowercase()
        .chars()
        .filter(|&c| c.is_alphanumeric())

    let rev_string: String = comp_string.chars()
        .rev()
        .collect();
    
    rev_string == comp_string
}
