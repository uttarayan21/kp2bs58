use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    keypair: PathBuf,
}

fn main() {
    let args = Args::parse();
    let keypair =
        fs::read(&args.keypair).unwrap_or_else(|_| panic!("Failed to read {:?}", &args.keypair));
    let keypair_array: Vec<u8> =
        serde_json::from_slice(&keypair).expect("Failed to deserialize keypair");
    let bs58_keypair = bs58::encode(keypair_array).into_string();
    println!("{}", bs58_keypair);
}
