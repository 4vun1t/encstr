use myobfuscator::{xstr, astr};

fn main() {
    let secret = xstr!("Hello from XOR obfuscation!");
    println!("{}", secret);

    let aes_secret = astr!("Hello from AES-128 obfuscation!");
    println!("{}", aes_secret);

    let multi_block = astr!("This is a much longer string that will span multiple AES blocks to demonstrate the encryption working correctly for longer messages.");
    println!("{}", multi_block);
}
