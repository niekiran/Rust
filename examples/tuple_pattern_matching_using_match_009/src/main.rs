fn main() {
    //imagine you get the point information from somewhere else
    let point = (0, 0);

    match point {
        // Pattern 1: Any first element and a negative second element.
        // `_` means we don't care about the value of the first element of the tuple
        (_, y) if y < 0 => {
            println!("Second element is negative:{}", y);
            println!("Take action 1");
        }

        // Pattern 2: Both elements of the tuple are exactly (0, 0)
        (0, 0) => {
            println!("Point is zero");
            println!("Take action 2");
        }

        // Default case: If none of the above patterns match
        _ => println!("All fine"),
    }
}
