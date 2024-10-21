//! A simple program that takes a number `n` as input, and computes the SHA256 hash `n` times.

use rsp_client_executor::io::ClientExecutorInput;
use rsp_client_executor::*;

fn main() {
    // Get the input slice from ziskos
    let input = include_bytes!("../../data/18884864.bin");
    let input = bincode::deserialize::<ClientExecutorInput>(input).unwrap();

    // Execute the block.
    let executor = ClientExecutor;
    let header = executor
        .execute::<EthereumVariant>(input)
        .expect("failed to execute client");
    let block_hash = header.hash_slow();

    println!("block_hash: {:?}", block_hash);
}
