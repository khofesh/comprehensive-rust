use rand::Rng;
use std::mem::{align_of, size_of};


/**
 * Enums
 */
fn generate_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>()
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

/**
 * variant payloads
 */
enum WebEvent {
    PageLoad, // variant without payload
    KeyPress(char), // tuple struct variant
    Click {x: i64, y: i64}, // full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::KeyPress(c) => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

fn second() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}

/**
 * enum sizes
 */

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
        stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

fn main() {
    println!("You got: {:?}", flip_coin());

    dbg_size!(Foo);

    second();
}
