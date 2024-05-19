fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;

    while index < numbers.len() {
        let number = numbers[index];

        if number % 2 == 0 {
            println!("{:2} is even", number);
        } else {
            println!("{:2} is odd", number);
        }

        index += 1;
    }
}
