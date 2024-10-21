use sha3::{Digest, Keccak256};

fn main() {
    let n: u64 = std::hint::black_box(10000);

    let mut out = [0u8; 32];
    for _ in 0..n {
        let mut hasher = Keccak256::new();
        hasher.update(out);
        let digest = &hasher.finalize();
        out = Into::<[u8; 32]>::into(*digest);
    }

    print!("n:{} {:?}\n", n, out);
}
