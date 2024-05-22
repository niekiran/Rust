fn fetch_data() -> (i32, String, bool) {
    let value = 42;
    let message = "All is well".to_string();
    let is_ok = true;

    (value, message, is_ok)
}

fn store_user_data(user_data: (i32, String, bool)) {
    println!("Storing user data: {:?}", user_data);
}

fn increment_elements(data: &mut (i32, i32, i32)) {
    data.0 += 1;
    data.1 += 1;
    data.2 += 1;
}

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

    //This function returns multiple values using tuple technique
    let (value, msg, is_ok) = fetch_data();

    //This code passes mutliple values to the function using tuple by value
    let user_data = (42, "Rahul".to_string(), false);
    store_user_data(user_data);

    //This code passes tuple by reference to another function
    let mut data = (1, 2, 3);
    increment_elements(&mut data);
    println!("{:?}", data);
}
