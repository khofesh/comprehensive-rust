// compile-time constants
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

// static
static BANNER: &str = "welcome to rustOS 3.14";

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);

    extra();

    let digest = compute_digest("hello");
    println!("Digest: {digest:?}");

    println!("{BANNER}");

    /* scopes and shadowing */
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

fn takes_u32(x: u32) {
    println!("u32: {x}")
}

fn takes_i8(y:i8) {
    println!("i8: {y}")
}

fn extra() {
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}
