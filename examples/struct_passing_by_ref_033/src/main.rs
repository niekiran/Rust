struct Person {
    name: String,
    age: u8,
}

//Function borrows an instance of the Person struct  mutably
fn update_person_age(person: &mut Person, new_age: u8) {
    //The dot operator implicitly dereferences the pointer and accesses the struct member.
    //So, we don't need to explicitly dereference the pointer using the * operator
    //before accessing the member.
    person.age = new_age;

    //It is the explicit way of dereferencing a pointer to a struct to access its field.
    //(*person).age = new_age;//OK but unconventional  
}

fn main() {
    let mut p = Person {
        name: String::from("Alice"),
        age: 25,
    };
    update_person_age(&mut p, 30);
    println!("Name: {}, Age: {}", p.name, p.age);
}
