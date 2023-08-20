use aes_crypto;

fn main() {
    println!("encrypt(5) = {}", aes_crypto::encrypt(5));
    println!("decrypt(99) = {}", aes_crypto::decrypt(99));
}
