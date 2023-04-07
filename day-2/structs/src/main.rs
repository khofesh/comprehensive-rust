

struct Person {
    name: String,
    age: u8,
}

fn first() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{} is {} years old", peter.name, peter.age);

    peter.age = 28;
    println!("{} is {} years old", peter.name, peter.age);

    let jackie = Person {
        name: String::from("jackie"),
        ..peter
    };
    println!("{} is {} years old", jackie.name, jackie.age);
}

/**
 * tuple structs
 */
struct Point(i32, i32);

fn second() {
    let p = Point(17, 23);
    println!("{}, {}", p.0, p.1);
}

/**
 * field shorthand syntax
 */
#[derive(Debug)]
struct Person2 {
    name: String,
    age: u8,
}

impl Person2 {
    fn new(name: String, age: u8) -> Person2 {
        Person2 {name, age}
    }
}

fn third() {
    let peter = Person2::new(String::from("peter"), 27);
    println!("{peter:?}");
    println!("name: {}", peter.name);
    println!("age: {}", peter.age);
}

fn main() {
    first();
    second();
    third();

}
