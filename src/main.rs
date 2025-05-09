use solana_sdk::signer::keypair::{Keypair, Signer};

fn main() {
    let key = Keypair::new();

    let pubkey = key.pubkey();
    let secret = key.to_base58_string();

    println!("Pubkey: {}", pubkey);
    println!("Secret: {}", secret);
}
