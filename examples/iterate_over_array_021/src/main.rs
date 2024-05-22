fn main() {
    let s = "Hello, World!";
    let target_char = 'o';
    let mut count = 0;

    for ch in s.chars() {
        if ch == target_char {
            count += 1;
        }
        
    }

    if count > 0 {
        println!("The character '{}' was found {} times in '{}'", target_char, count, s);
    } else {
        println!("The character '{}' was not found in '{}'", target_char, s);
    }
}