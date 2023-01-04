fn main() {
    let mut array: [i8;10] = [42;10];
    array[5] = 0;
    println!("a: {:?}", array);

    let tupple: (i8, bool) = (7, true);
    println!("1st index: {}", tupple.0);
    println!("2nd index: {}", tupple.1);
}
