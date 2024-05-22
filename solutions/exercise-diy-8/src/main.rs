fn is_palindrome(chars: &[char]) -> bool {
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }

    true
}

fn main() {
    let char_array_1 = ['r', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_1) {
        println!("given array is a palindrome.");
    } else {
        println!("given array is not a palindrome");
    }

    let char_array_2 = ['b', 'a', 'd', 'a', 'r'];
    if is_palindrome(&char_array_2) {
        println!("given array is a palindrome.");
    } else {
        println!("given array is not a palindrome");
    }
}
