fn main() {
    //
    // print floating point values with custom decimal places
    //
    let real_value = 3.14159;
    println!("With 2 decimal places value would be {:.2}", real_value);
    println!("With 6 decimal places value would be {:.6}", real_value);
    println!(
        "int part of the real value {} is {}",
        real_value,
        real_value as i32 // 'as' does casting
    );

    //
    // print in hexa decimal style
    //
    let decimal_num = 6789;
    let output1 = format!(
        "decimal number {} in hex is {:#X}",
        decimal_num, decimal_num
    );

    let output2 = format!(
        "decimal number {} in hex is {:#x}",
        decimal_num, decimal_num
    );

    let output3 = format!("decimal number {} in hex is {:x}", decimal_num, decimal_num);

    println!("{}\n{}\n{}", output1, output2, output3);

    //
    // print in binary
    //
    println!("decimal number {num} in binary is {num:#b}", num = 6789);
    println!("decimal number {} in binary is {:#b}", 500, 500);
}
