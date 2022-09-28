fn main() {
    let string = String::from("Hello");
    println!("Last char of {} is {}", string.clone(), last_char(string));
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return '🚨';
    }
    string.chars().next_back().unwrap()
}
