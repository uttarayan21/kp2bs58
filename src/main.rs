use clap::{IntoApp, Parser};
use ed25519_dalek::Keypair;
use std::{fs, io, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    keypair: Option<PathBuf>,
    #[clap(short, long)]
    decode: bool,
    #[clap(short, long, conflicts_with = "decode")]
    verbose: bool,
}

fn print_verbose(keypair: impl AsRef<[u8]>) {
    // Keypair doesn't have std::fmt::Display
    let keypair =
        Keypair::from_bytes(keypair.as_ref()).expect("Unable to build a Keypair from data");
    // Convert it to bytes and encode it to bs58 string
    let keypair_bs58 = bs58::encode(keypair.to_bytes()).into_string();
    let pubkey_bs58 = bs58::encode(keypair.public.to_bytes()).into_string();
    let privkey_bs58 = bs58::encode(keypair.secret.to_bytes()).into_string();

    println!("Keypair {}", keypair_bs58);
    println!("Pubkey  {}", pubkey_bs58);
    println!("Privkey {}", privkey_bs58);
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let tty_is_stream = !atty::is(atty::Stream::Stdin);

    match (args.decode, args.keypair, tty_is_stream) {
        (false, Some(keypair_path), false) => {
            // kp2bs58 -k ~/.config/solana/id.json
            let keypair = fs::read(&keypair_path)
                .unwrap_or_else(|_| panic!("Failed to read {:?}", &keypair_path));
            let keypair_array: Vec<u8> =
                serde_json::from_slice(&keypair).expect("Failed to deserialize keypair");
            let bs58_keypair = bs58::encode(&keypair_array).into_string();

            match args.verbose {
                true => print_verbose(&keypair_array),
                false => println!("{}", bs58_keypair),
            }
        }
        (false, None, true) => {
            // cat ~/.config/solana/id.json | kp2bs58
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let keypair_array: Vec<u8> =
                serde_json::from_str(&buffer).expect("Failed to deserialize keypair");
            let bs58_keypair = bs58::encode(&keypair_array).into_string();
            match args.verbose {
                false => println!("{}", bs58_keypair),
                true => print_verbose(&keypair_array),
            }
        }
        (true, Some(keypair_path), false) => {
            let keypair_u8 = fs::read(&keypair_path)
                .unwrap_or_else(|_| panic!("Failed to read {:?}", &keypair_path));
            let keypair_bs58 =
                String::from_utf8(keypair_u8).expect("Unable to read the keypair in utf8 encoding");
            let keypair_array = bs58::decode(keypair_bs58)
                .into_vec()
                .expect("Failed to decode keypair");
            let keypair_json =
                serde_json::to_string(&keypair_array).expect("Failed to seialize keypair to json");
            println!("{}", keypair_json);
        }
        (true, None, true) => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let keypair_array = bs58::decode(buffer)
                .into_vec()
                .expect("Failed to decode keypair");
            let keypair_json =
                serde_json::to_string(&keypair_array).expect("Failed to seialize keypair to json");
            println!("{}", keypair_json);
        }
        (_, _, _) => {
            Args::into_app()
                .print_long_help()
                .expect("Failed to print help");
        }
    }
    Ok(())
}
