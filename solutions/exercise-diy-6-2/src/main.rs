/*
   Convert 32-bit RGBA8888 color format to 16-bit RGB565 color Format
   Write a program which accepts 32-bit RGBA8888 color format in hex  from the user
   and converts that into 16-bit RGB565 color format

   Hints
   ============================
   1) Extract the red, green, and blue components from the 32-bit RGBA input.
   2) Scale down these color components to fit the RGB565 format,
    which allocates 5 bits for red and blue, and 6 bits for green.
    For example if 0xABCDEFEE is in RGBA8888 format, in binary it would look like below
    10101011(R) 11001101(G) 11101111(B) 11101110(A)
    to convert this into RGB565,
            i) Neglect A
            ii) in R consider only most significant 5 bits
            iii) in G consider only most significant 6 bits
            iv) in B consider only most significant 5 bits

   3) Removing Hex prefix from user input:
        Use the 'trim_start_matches' method on the string to remove the "0x" or "0X" prefix
   4) To convert String to Integer U32 value , explore
        u32::from_str_radix() with radix = 16


   Expected Output
   =============================
   Enter RGBA8888 data in hex format:0xABCDEFEE
   0xABCDEFEE RGB565 equivalent is 0xAE7D

*/

use std::io;
//We are using methods(like flush()) from the 'Write' trait of the std::io module
use std::io::Write;

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn parse_hex_to_u32(hex_str: &str) -> u32 {
    let clean_hex_str = hex_str.trim_start_matches("0x").trim_start_matches("0X");
    u32::from_str_radix(clean_hex_str, 16).expect("Failed to parse hex")
}

fn rgba8888_to_rgb565(rgba: u32) -> u16 {
    // Extract red, green, and blue components
    let red = (rgba >> 24) & 0xFF;
    let green = (rgba >> 16) & 0xFF;
    let blue = (rgba >> 8) & 0xFF;

    // Convert to RGB565
    let red_565 = (red >> 3) << 11;
    let green_565 = (green >> 2) << 5;
    let blue_565 = blue >> 3;

    (red_565 | green_565 | blue_565) as u16
}

fn main() {
    print!("Enter RGBA8888 data in hex format:");
    //flushes the buffer of the standard output
    io::stdout().flush().expect("Failed to flush stdout");

    let input = read_user_input();
    let rgba888 = parse_hex_to_u32(&input);
    let rgb565 = rgba8888_to_rgb565(rgba888);
    println!("{:#x} in rgb565 format is {:#x}", rgba888, rgb565);
}
