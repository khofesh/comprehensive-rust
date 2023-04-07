use rand::Rng;

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

fn main() {
    println!("You got: {:?}", flip_coin());
}
