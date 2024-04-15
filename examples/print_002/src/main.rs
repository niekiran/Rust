fn main() {
    print!("Hello, ");
    eprintln!("An error occurred: invalid input");
    let name = "John";
    let age = 30;
    let message = format!("My name is {} and I am {} years old",name,age);
    println!("{}",message);
    println!("Hello, World!");

    //
    // named placeholders 
    //
    let message = format!("My name is {user_name} and \
                                I am {user_age} years old",user_age=age,user_name=name);
    println!("{}",message);

    //
    // print floating point values with custom decimal places 
    //
    let real_value = 3.14159;
    println!("With 2 decimal places value would be {:.2}",real_value);
    println!("With 6 decimal places value would be {:.6}",real_value);
    println!("int part of the real value {} is {}",real_value, real_value as i32); // 'as' does casting 

    //
    // print in hexa decimal style
    //
    let decimal_num = 6789;
    let output1 = format!("decimal number {} in hex is {:#X}",decimal_num,decimal_num);
    let output2 = format!("decimal number {} in hex is {:#x}",decimal_num,decimal_num);
    let output3 = format!("decimal number {} in hex is {:x}",decimal_num,decimal_num);
    println!("{}\n{}\n{}",output1,output2,output3);

    //
    // print in binary 
    //
    println!("decimal number {num} in binary is {num:#b}",num=6789);
    println!("decimal number {} in binary is {:#b}",500,500);

    //
    // print a message which contains special
    // characters like double quotes("blah blah") and backslash('\')
    // 
    //println!("David says, "Programming is fun"");//Error
    println!("David says, \"Programming is fun\""); //OK. Note that \ used to help compiler escape ' " ' 

    //println!("C:\My computer\My folder");//Error
    println!("C:\\My computer\\My folder");//OK. '\' used to help compiler escape '\' 

    //
    // raw string example
    // 

    /* Works because in 'r' tagged string , escape character '\' is not recognized */
    println!(r"C:\My computer\My folder");
    let message = r"\ \ \ \ Today is holiday \ \ \ \";
    println!("{}",message);

    /* Error. Because 'r' tagged string cannot contain double quotes */
    //println!(r"This is a triple quoted string """ This month has 30 days """  ");

    /* Error because in 'r' tagged string escape character '\' is not recognized */
    //println!(r"This is a triple quoted string \"\"\" This month has 30 days \"\"\"  ");

    //
    //string tagging with r#.......# 
    //
    /* #### used for readability purpose , you can use as many #s you want */
    let mut _message = r####"\ \ \ \ "Today is holiday" \ \ \ \"####;
    _message = r#"\ \ \ \ "Today is holiday" \ \ \ \"#;/* same as above */
    println!("{}",message);

}







