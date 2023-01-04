struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {a} else {b}
}

fn main() {
    println!("copy me!");

    /* references */
    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;

    *ref_x = 20;
    println!("x: {x}");

    /* dangling references */
    // let dangling_x: &i32;
    // {
    //     let dang_x: i32 = 10;
    //     dangling_x = &dang_x;
    // }
    // println!("dangling_x: {dangling_x}");

    /* slices */
    let a_slice: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a_slice: {a_slice:?}");

    let s_slice: &[i32] = &a_slice[2..4];
    println!("s_slice: {s_slice:?}");

    /* string vs str */
    let s1: &str = "hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    /* functions */
    fizzbuzz_to(20);

    /* methods */
    let mut rect = Rectangle {
        width: 10,
        height: 5
    };

    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());

    /* generics */
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn fizzbuzz(n: u32) -> () {
    match (is_divisible_by(n, 3), is_divisible_by(n, 5)) {
        (true, true) => println!("fizzbuzz"),
        (true, false) => println!("fizz"),
        (false, true) => println!("buzz"),
        (false, false) => println!("{n}"),
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
