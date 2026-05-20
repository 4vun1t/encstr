use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("keys.rs");

    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut state = seed as u64;

    let mut next_u8 = move || -> u8 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state as u8
    };

    let mut next_u32 = move || -> u32 {
        let b0 = next_u8() as u32;
        let b1 = next_u8() as u32;
        let b2 = next_u8() as u32;
        let b3 = next_u8() as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    };

    let xor_key = next_u8();
    let mut aes_key = [0u8; 16];
    for b in &mut aes_key {
        *b = next_u8();
    }
    let entangle_seed = next_u32();

    let fmt_hex = |v: &[u8]| -> String {
        v.iter().map(|b| format!("0x{:02x}", b)).collect::<Vec<_>>().join(", ")
    };

    let content = format!(
        "pub const XOR_KEY: u8 = 0x{:02x};\npub const AES_KEY: [u8; 16] = [{}];\npub const ENTANGLE_SEED: u32 = 0x{:08x};\n",
        xor_key,
        fmt_hex(&aes_key),
        entangle_seed,
    );

    fs::write(&dest_path, content).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
