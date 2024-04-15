use std::time::Duration;

use base58::ToBase58;
use uuid::Uuid;
pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;

fn main() {
    for _ in 0..3 {
        let uuid = Uuid::new_v4();
        println!("{:>12}: {uuid}", "uuid v4");
    }
    println!();
    for _ in 0..3 {
        let uuid = Uuid::now_v7();
        std::thread::sleep(std::time::Duration::from_millis(1));
        println!("{:>12}: {uuid}", "uuid v7");
    }

    // encoding
    let uuid = Uuid::now_v7();
    println!("{:>12} ({}): {uuid}", "uuid v7", uuid.to_string().len());
    let b64 = data_encoding::BASE64.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b64}", "b64", b64.len());

    let b64 = data_encoding::BASE64URL_NOPAD.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b64}", "b64", b64.len());

    // alphabetical order - can use on sqlite
    let b32hex = data_encoding::BASE32HEX_NOPAD.encode(uuid.as_bytes());
    println!("{:>12} ({}): {b32hex}", "b32hex", b32hex.len());

    // alphanumber upper lower
    let b58 = uuid.as_bytes().to_base58();
    println!("{:>12} ({}): {b58}", "b58", b58.len());

    println!();
}
