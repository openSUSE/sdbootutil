use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::sign::Signer;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Digest used to calculate the HMAC
    digest: Digest,

    /// File with the secret key as an hex string
    key: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Digest {
    SHA1,
    SHA256,
    SHA384,
    SHA512,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let key = PKey::hmac(
        &hex::decode(fs::read_to_string(&cli.key)?.trim()).expect("valid hexadecimal string"),
    )
    .expect("valid HMAC key");
    let mut message = String::new();
    io::stdin().read_to_string(&mut message)?;

    let mut signer;
    let result = match cli.digest {
        Digest::SHA1 => {
            signer = Signer::new(MessageDigest::sha1(), &key).unwrap();
            signer.update(message.as_bytes()).unwrap();
            hex::encode(signer.sign_to_vec().unwrap())
        }
        Digest::SHA256 => {
            signer = Signer::new(MessageDigest::sha256(), &key).unwrap();
            signer.update(message.as_bytes()).unwrap();
            hex::encode(signer.sign_to_vec().unwrap())
        }
        Digest::SHA384 => {
            signer = Signer::new(MessageDigest::sha384(), &key).unwrap();
            signer.update(message.as_bytes()).unwrap();
            hex::encode(signer.sign_to_vec().unwrap())
        }
        Digest::SHA512 => {
            signer = Signer::new(MessageDigest::sha512(), &key).unwrap();
            signer.update(message.as_bytes()).unwrap();
            hex::encode(signer.sign_to_vec().unwrap())
        }
    };

    println!("{result}");

    Ok(())
}
