struct Point(i32, i32);

#[derive(Copy, Clone, Debug)]
struct Point2(i32, i32);

#[derive(Debug)]
struct Point3(i32, i32);

#[derive(Debug)]
struct Point4(i32, i32);

#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }

    // println!("y: {}", p.1);

    /* move semantics */
    let s1: String = String::from("Hello");
    let s2: String = s1;
    println!("s2: {s2}");

    /* moves in function calls */
    let name = String::from("Alice");
    say_hello(name);

    /* copying and cloning */
    let p1 = Point2(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    /* borrowing */
    let b1 = Point3(3, 4);
    let b2 = Point3(10, 20);
    let b3 = add(&b1, &b2);
    println!("{b1:?} + {b2:?} = {b3:?}");

    /* shared and unique borrows */
    let mut a: i32 = 10;
    let b: &i32 = &a;

    println!("b: {b}");
    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");

    /* lifetimes in function calls */
    let c1: Point4 = Point4(10, 10);
    let c2: Point4 = Point4(20, 20);
    let c3: &Point4 = left_most(&c1, &c2);
    println!("left-most point: {:?}", c3);

    /* lifetimes in data structures */
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    // erase(text);
    println!("{fox:?}");
    println!("{dog:?}");
}

fn say_hello(name: String) {
    println!("hello {name}")
}

fn add(p1: &Point3, p2: &Point3) -> Point3 {
    Point3(p1.0 + p2.0, p1.1 + p2.1)
}

fn left_most<'a>(p1: &'a Point4, p2: &'a Point4) -> &'a Point4 {
    if p1.0 < p2.0 {
        p1
    } else {
        p2
    }
}

fn erase(text: String) {
    println!("bye {text}!")
}
