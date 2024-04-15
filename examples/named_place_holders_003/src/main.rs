fn main() {
    let name = "John";
    let age = 30;

    //palce holders( '{}' ) with out names
    let msg = format!("I am {} and I am {} years old", name, age);

    //place holders( '{}' ) with given names 'user_name' and 'user_age'
    let msg = format!(
        "I am {user_name} and I am {user_age} years old",
        user_name = name,
        user_age = age
    );
}
