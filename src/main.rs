use std::str::FromStr;
use std::env;
use walletcryptography::app::Network;
use walletcryptography::crypto::*;
use std::fs::File;
use std::io::{Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let network = Network::from_str(args[1].clone().as_str()).expect("network");
    let raw_pr_key = args[2].clone().to_lowercase();

    assert!(raw_pr_key.len() == 64, "private-key");
    assert!(raw_pr_key.chars().into_iter().all(|c| u8::from_str_radix(c.to_string().as_str(), 16).unwrap() < 16), "private-key");

    // derive pub key
    let pub_key = secp256k1::get_public_key(&raw_pr_key);

    // get address
    let address: String;
    match network {
        Network::Bitcoin => {
            address = bitcoin::derive_compressed_address(&pub_key);
        },
        Network::Ethereum => {
            address = ethereum::derive_address(&pub_key);
        }
    }

    let pr_key = if network == Network::Bitcoin {
        bitcoin::encode_compressed_pr_key(&raw_pr_key)
    } else {
        raw_pr_key
    };

    let mut output = File::create(format!("./{}_address_{}", network.to_string(), address)).expect("file-creation");
    write!(&mut output,
        "[Generate New Wallet Private/Public Keys and Address]\n\nNetwork: {}\nAddress: {}\nPrivate Key[*]: {}\nUncompressed Public Key: {}\n\n[*] Keep secret at all times.",
        network.to_string(),
        address,
        pr_key,
        pub_key
    ).expect("write-file");
}


