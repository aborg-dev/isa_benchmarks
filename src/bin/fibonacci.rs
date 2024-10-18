use std::convert::TryInto;

fn main() {
    let input: Vec<u8> = std::hint::black_box(vec![16, 39, 0, 0, 0, 0, 0, 0]);

    // Convert the byte array to a u64 number
    let n: u64 = u64::from_le_bytes(input.try_into().unwrap());

    let mut a: u64 = 1;
    let mut b: u64 = 1;
    for _ in 0..n - 1 {
        (a, b) = (b, a.wrapping_add(b));
    }

    print!("n:{} {}\n", n, b);
}
