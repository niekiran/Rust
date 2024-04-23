use std::io;

/*
    Convert seconds to HH:MM:SS format
*/

fn main() {
    
    let total_seconds = get_user_input();
    println!(
        "Time in 24hr format is: {}",
        convert_seconds_to_24hr_format(total_seconds)
    );
}


fn convert_seconds_to_24hr_format(total_seconds: u32) -> String {
    if total_seconds > 86399 {
        panic!("Your input should be between 0 to 86,399 ");
    }

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    let format_24hr = format!(
        "{:02}:{:02}:{:02}",
        hours, minutes, seconds
    );

    return format_24hr; //String
}


fn read_from_stdin() -> String {
    let mut input = String::new();

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
}

fn parse_string_as_u32(input: String) -> u32 {
    let total_seconds: u32 = input
        .trim()
        .parse()
        .expect("Input number only without any sign!");

    return total_seconds;
}


fn get_user_input() -> u32 {

    return parse_string_as_u32(read_from_stdin());
}

#[cfg(test)]
mod tests {

    mod time_format {
        //keep all the test cases to unit test convert_seconds_to_24hr_format
        //1. when total_seconds is zero, function must return 00:00:00
        #[test]
        fn test_when_total_seconds_is_zero_expected_00_00_00() {
            assert_eq!("00:00:00", super::super::convert_seconds_to_24hr_format(0));
        }

        //2. when total_seconds is 86400, function must panic 
        #[test]
        #[should_panic(expected = "should be between 0 to 86,399")]
        fn test_when_total_seconds_is_86400_function_must_panic() {
            super::super::convert_seconds_to_24hr_format(86400);
        }

        //3. when total_seconds is 86399, function  must return 23::59::59



    }

    mod parse_user_input {
        //keep all the test cases to unit test get_user_input
        //1) user enters 0, function must return 0
        #[test]
        fn test_user_enters_zero_function_must_return_zero() {
            assert_eq!(0, super::super::parse_string_as_u32("0\n\r".to_string()));
        }
        //2) when user enters whole number, functin must return whole number 
        //3) when user enters negative number,function must panic 
        //4) when user enters decimal number, function must panic 
        //5) when user eners any  characters other than digits, function must panic 
        //6) when user enters nothing(just hits enter key), function must panic 
        //7) when user enters a number which is greater than max value of u32, function must panic 
        #[test]
        #[ignore]
        #[should_panic]
        fn test_user_enters_number_greater_than_max_u32_function_must_panic() {
            let some_big_number_as_string = "4294967296\n\r".to_string();
            super::super::parse_string_as_u32(some_big_number_as_string);
        }

    }

}