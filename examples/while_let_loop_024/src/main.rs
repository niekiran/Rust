fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut iterator = numbers.iter();

    //just an example of using while let, for loop suits  here.
    while let Some(number) = iterator.next() {
        if number % 2 == 0 {
            println!("{:2} is even", number);
        } else {
            println!("{:2} is odd", number);
        }
    }
}
