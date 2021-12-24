use clap::{IntoApp, Parser};
use std::{fs, io, path::PathBuf};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    keypair: Option<PathBuf>,
    #[clap(short, long)]
    decode: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if !args.decode {
        if let Some(keypair_path) = args.keypair {
            let keypair = fs::read(&keypair_path)
                .unwrap_or_else(|_| panic!("Failed to read {:?}", &keypair_path));
            let keypair_array: Vec<u8> =
                serde_json::from_slice(&keypair).expect("Failed to deserialize keypair");
            let bs58_keypair = bs58::encode(keypair_array).into_string();
            println!("{}", bs58_keypair);
        } else {
            if atty::is(atty::Stream::Stdin) {
                Args::into_app()
                    .print_long_help()
                    .expect("Failed to print help");
                return Ok(());
            }
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let keypair_array: Vec<u8> =
                serde_json::from_str(&buffer).expect("Failed to deserialize keypair");
            let bs58_keypair = bs58::encode(keypair_array).into_string();
            println!("{}", bs58_keypair);
        }
        Ok(())
    } else {
        if atty::is(atty::Stream::Stdin) {
            println!("The text must be piped into this program");
            Args::into_app()
                .print_long_help()
                .expect("Failed to print help");
            return Ok(());
        }
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let keypair_array = bs58::decode(buffer)
            .into_vec()
            .expect("Failed to decode your key");
        let keypair =
            serde_json::to_string(&keypair_array).expect("Failed to seialize keypair to json");
        println!("{}", keypair);
        Ok(())
    }
}
