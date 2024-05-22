fn main() {
    let rcvd_data = (5, "hello", 8);

    //pattern to Process the middle element only when the first element is
    //greater than 0, the last element is less than 10
    match rcvd_data {
        (a, s, c) if a > 0 && c < 10 => {
            println!("Valid data: s = {}", s);
        }

        _ => println!("Invalid data"),
    }

    //match any tuple with the middle element equal to "SOP",
    //regardless of the values of the first and third elements

    let rcvd_data = (5, "SOP", 8);

    match rcvd_data {
        (_, "SOP", _) => println!("Middle element is 'SOP'"),
        _ => println!("Middle element is not 'SOP'"),
    }

    //match against specific elements of specific value
    let rcvd_data = (1, "hello", true);

    match rcvd_data {
        (1, s, true) => println!("The second element is {}", s),
        _ => (),
    }

    //Tuple match with range expression
    let rcvd_data = (1, 2, 20);
    //can you try this using if let instead?
    match rcvd_data {
        /*
           The @ symbol followed by a range expression is used in pattern matching to
           create a binding to the value of the pattern being matched, while also
           checking that the matched value is within the specified range.
        */
        (_, _, c @ 10..=20) => {
            println!("c is between 10 and 20");
            println!("C is {}", c)
        }
        _ => (),
    }

    //Pattern matching with tuple using the
    //rest operator( .. )  to ignore some elements
    let rcvd_data = (1, "hello", 2.5, true, 'a');

    match rcvd_data {
        (_, _, c, ..) if c > 2.0 => println!("The third element is greater than 2.0"),
        _ => println!("The third element is less than or equal to 2.0"),
    }

    let rcvd_data = (10, "hello", 2.5, true, 'a');

    match rcvd_data {
        (a @ 10, b @ "hello", ..) => println!("The first and second element: {} and {}.", a, b),
        _ => println!("The tuple does not match the pattern."),
    }

    //Avoid move while matching using 'ref'
    let the_date = (
        "Monday".to_string(),
        "25".to_string(),
        "June".to_string(),
        "2023".to_string(),
    );

    /*ref keyword used here to avoid moving values during pattern matching.
    By using ref, you can borrow the value instead of moving it. */
    match the_date {
        (ref day, ..) if day == "Sunday" => {
            println!("Its Sunday");
        }

        _ => println!("Someother day"),
    }

    println!("{:?}", the_date);
}
