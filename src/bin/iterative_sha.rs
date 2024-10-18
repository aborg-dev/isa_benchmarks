use sha2::{Digest, Sha256};
use std::convert::TryInto;

fn main() {
    let input: Vec<u8> = std::hint::black_box(vec![16, 39, 0, 0, 0, 0, 0, 0]);

    // Convert the byte array to a u64 number
    let n: u64 = u64::from_le_bytes(input.try_into().unwrap());

    let mut out = [0u8; 32];

    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(out);
        let digest = &hasher.finalize();
        out = Into::<[u8; 32]>::into(*digest);
    }

    print!("n:{} {:?}\n", n, out);
}
