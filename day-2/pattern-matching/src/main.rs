/**
 * destructuring enums
 */
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n/2) 
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

/**
 * destructuring structs
 */
struct Foo {
    x: (u32, u32),
    y: u32,
}


#[rustfmt::skip]
fn main() {
    let input = 'x';

    match input {
        'q' => println!("quitting!"),
        'a' | 's' | 'w' | 'd' => println!("moving around"),
        '0'..='9' => println!("number input"),
        _ => println!("something else"),
    }

    // destructuring
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }

    // destructuring structs
    let foo = Foo {x: (1, 2), y: 3};
    match foo {
        Foo {x: (1, b), y} => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo {y: 2, x: i } => println!("y = 2, x = {i:?}"),
        Foo {y, ..} => println!("y = {y}, other fields were ignored"),
    }

    // destructuring arrays
    let triple = [0, -2, 3];
    println!("tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("first is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("first is 1 and the rest were ignored"),
        _ => println!("all elements were ignored"),
    }

    // match guards
    let pair = (2, -2);
    println!("tell me about {pair:?}");
    match pair {
        (x, y) if x == y => println!("these are twins"),
        (x, y) if x + y == 0 => println!("antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("the first one is odd"),
        _ => println!("no correlation..."),
    }
}
