use std::io;

/*
    Convert seconds to HH:MM:SS format
*/

fn main() {
    let mut input = String::new();

    println!("Enter the time of the day in seconds(0 to 86,399):");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let total_seconds: u32 = input
        .trim()
        .parse()
        .expect("Input number only without any sign!");

    if total_seconds > 86399 {
        panic!("Your input should be between 0 to 86,399 ");
    }

    let hours = total_seconds / 3600;
    let remaining_seconds = total_seconds % 3600;
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;

    println!(
        "Time in 24hr format is: {:02}:{:02}:{:02}",
        hours, minutes, seconds
    );
}
