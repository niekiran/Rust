fn main() {
    //A tuple is a heterogeneous data type, that means it can hold elements of different types.
    let my_tuple = (1, "hello", true);

    //elements of a tuple are accessed using index numbers
    let my_int = my_tuple.0;
    let my_string = my_tuple.1;
    let my_bool = my_tuple.2;

    //or

    //Tuple destructuring
    let (my_int, my_string, my_bool) = my_tuple;

    //to print tuple use {:?} format specifier because Tuples dont implement the Display trait
    println!("{:?}", my_tuple);

    //mutable tuple
    let mut my_mut_tuple = (1, "Hello World", false);
    my_mut_tuple.1 = "All is well";
    println!("{:?}", my_mut_tuple);
}
