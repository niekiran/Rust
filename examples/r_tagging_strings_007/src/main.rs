fn main() {
    //
    // print a message which contains special
    // characters like double quotes("blah blah") and backslash('\')
    //
    //println!("David says, "Programming is fun"");//Error
    println!("David says, \"Programming is fun\""); //OK. Note that \ used to help compiler escape ' " '

    //println!("C:\My computer\My folder");//Error
    println!("C:\\My computer\\My folder"); //OK. '\' used to help compiler escape '\'

    //
    // raw string example
    //
    /* Works because in 'r' tagged string , escape character '\' is not recognized */
    println!(r"C:\My computer\My folder");
    let message_1 = r"\ \ \ \ Today is holiday \ \ \ \";
    println!("{}", message_1);

    /* Error. Because 'r' tagged string cannot contain double quotes */
    //println!(r"This is a triple quoted string """ This month has 30 days """  ");

    /* Error because in 'r' tagged string escape character '\' is not recognized */
    //println!(r"This is a triple quoted string \"\"\" This month has 30 days \"\"\"  ");

    //
    //string tagging with r#.......#
    //
    /* #### used for readability purpose , you can use as many #s you want */
    let message_2 = r####"\ \ \ \ "Today is holiday" \ \ \ \"####;
    let _message_3 = r#"\ \ \ \ "Today is holiday" \ \ \ \"#; /* same as above */
    println!("{}", message_2);
}
