//Compiler will automatically derive the Default trait for this struct
//This is particularly useful for initializing
//structs with default values without having to specify each field manually
#[derive(Default, Debug)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
    height: f32,
    initial: char,
}

#[derive(Debug)]
struct Process {
    name: String,
    pid: u32,
    group: String,
}

fn main() {
    //Default trait provides 'default()' associated function
    let person = Person::default();
    println!("Person: {:?}", person);

    //Belo code shows initialization of an instance using another instance
    let process1 = Process {
        name: String::from("Ping"),
        pid: 0x1234,
        group: String::from("Networking"),
    };
    println!("Process 1: {:#X?}", process1);

    // Create a new process by updating the name field
    let process2 = Process {
        name: String::from("Route"),
        ..process1 //other member fields of the 'process2' takes values from 'process1'
    };
    println!("process 2: {:#X?}", process2);

    // Create a new process by updating the pid and group fields
    let process3 = Process {
        pid: 0x3456,
        group: String::from("Security"),
        ..process2
    };
    println!("process 3: {:#X?}", process3);
}
