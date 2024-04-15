fn main() {
    let name = "John";
    let age = 30;

    //palce holders( '{}' ) without names
    let msg = format!("I am {} and I am {} years old", name, age);

    //place holders( '{}' ) with given names 'user_name' and 'user_age'
    let msg = format!(
        "I am {user_name} and I am {user_age} years old",
        user_name = name,
        user_age = age
    );//This code can also be written like below 

    // You can use a shorthand syntax for variables in 
    // formatted strings when the variable names are the same as the names of the 
    // placeholders
    let msg = format!(
        "I am {name} and I am {age} years old"
    );

    println!("{}", msg);

}
