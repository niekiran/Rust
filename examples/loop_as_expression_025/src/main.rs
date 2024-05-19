fn main() {
    let numbers = [1, 2, 3, -4, 5];

    let message = 'outer: loop {
        for n in numbers {
            if n < 0 {
                break 'outer "Invalid Array";
            }
        }
        break 'outer "Valid Array";
    };

    println!("{message}");
}
