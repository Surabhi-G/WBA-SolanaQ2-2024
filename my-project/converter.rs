// extern crate bs58;

// use std::io::{self, Write};

// fn main() {
//     println!("Enter your private key (byte vector):");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read input");

//     let private_key: Vec<u8> = input
//         .trim()
//         .split(',')
//         .map(|x| x.trim().parse().expect("Invalid byte value"))
//         .collect();

//     let base58_key = bs58::encode(private_key).into_string();
//     println!("Base58-encoded private key: {}", base58_key);
// }

use bs58;
// use std::io::{self, BufRead};

fn main() {
    println!("Enter a base58 string:");
    // let stdin = io::stdin();
    let base58 = "YOUR_PK_HERE"; //stdin.lock().lines().next().unwrap().unwrap();

    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("Converted to Vec<u8>: {:?}", wallet);

    let base58 = bs58::encode(wallet).into_string();
    println!("Converted back to base58: {:?}", base58);
}