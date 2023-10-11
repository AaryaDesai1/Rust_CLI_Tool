use caeser_cipher_cli::{decrypt, encrypt};
use clap::Parser;

/// CLI tool to encrypt and decrypt messages using the caeser cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Encrypt the message
    #[arg(short, long)]
    encrypt: bool,

    /// decrypt the message
    #[arg(short, long)]
    decrypt: bool,

    /// The message to encrypt or decrypt
    #[arg(short, long)]
    message: String,

    /// The shift to use for the cipher
    /// Must be between 1 and 25, the default is 3
    #[arg(short, long, default_value = "3")]
    shift: u8,
}

// run it
fn main() {
    let args = Args::parse();
    if args.encrypt {
        println!("{}", encrypt(&args.message, args.shift));
    } else if args.decrypt {
        println!("{}", decrypt(&args.message, args.shift));
    } else {
        println!("Please specify either --encrypt or --decrypt");
    }
}
