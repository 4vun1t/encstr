use encstr::{xstr, astr, cobl, entangle};

fn real_plus_one(x: i32) -> i32 { x.wrapping_add(1) }
fn decoy_minus_seven(x: i32) -> i32 { x.wrapping_sub(7) }
fn real_times_three(x: i32) -> i32 { x.wrapping_mul(3) }
fn decoy_div_two(x: i32) -> i32 { x.wrapping_div(2) }

fn main() {
    let secret = xstr!("Hello from XOR obfuscation!");
    println!("{}", secret);

    let aes_secret = astr!("Hello from AES-128 obfuscation!");
    println!("{}", aes_secret);

    let multi_block = astr!("This is a much longer string that will span multiple AES blocks to demonstrate the encryption working correctly for longer messages.");
    println!("{}", multi_block);

    // ── Control Flow Obfuscation ──────────────────────────────────────────
    let val = cobl!({
        let x = 42;
        x * 2
    });
    println!("cobl! result: {}", val);

    // ── CFG Entanglement ──────────────────────────────────────────────────
    // Entangle unrelated logic paths into a single dispatch
    // Each funclet is a non-capturing closure (coerces to fn pointer)
    let results: Vec<i32> = entangle!([
        || -> i32 { real_plus_one(10) },
        || -> i32 { decoy_minus_seven(10) },
        || -> i32 { real_times_three(10) },
        || -> i32 { decoy_div_two(10) },
    ]);
    println!("entangle! results (permuted by ENTANGLE_SEED): {:?}", results);
}
