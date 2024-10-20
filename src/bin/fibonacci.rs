fn main() {
    let n: u64 = std::hint::black_box(1000000);

    let mut a: u64 = 1;
    let mut b: u64 = 1;
    for _ in 0..n - 1 {
        (a, b) = (b, a.wrapping_add(b));
    }

    print!("n:{} {}\n", n, b);
}
