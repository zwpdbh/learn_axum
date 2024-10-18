#![allow(unused)] // For beginning only.
use anyhow::{Ok, Result};
use rand::RngCore;

// cargo run --example gen_key
pub fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits = 64 bytes
    let _ = rand::thread_rng().fill_bytes(&mut key);
    println!("\nGenerated key for HMAC:\n{key:?}");

    let b64url = base64_url::encode(&key);
    println!("\nKey b64u encode:\n{b64url}");

    Ok(())
}
