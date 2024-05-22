//Defining a struct
/*
   1. use PascalCase for naming structs(e.g, UserProfile , Point3D , HttpRequest , OrderDetails )
   2. use snake case for naming member fields(e.g, age, last_name, is_valid, alt_number)
*/
struct Person {
    name: String,
    age: u32,
    address: String,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    //creating an instance of the 'struct Person'
    let person = Person {
        name: String::from("Alice"),
        age: 25,
        address: String::from("123 Main St"),
    };

    //use the dot (.) operator to print the values of each member element
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Address: {}", person.address);

    //example of field initialization short hand method
    let x = 10; //this variable used to intialize the field name of below struct instance
    let y = 0; //this variable used to intialize the field name of below struct instance

    let point = Point {
        x, // compier sees it as x: x i.e field_name: variable_name
        y, // compier sees it as y: y
    };

    println!("{:?}", point);

    //Struct is move by default . consider the below code
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1; // p1 is moved to p2
                 // println!("{:?}", p1); //Error. To make it work derive Copy and Clone trait (#[derive(Debug, Copy, Clone)])
}
