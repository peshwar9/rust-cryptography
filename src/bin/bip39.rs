use bip39::Mnemonic;
use rand::{thread_rng, RngCore};

// Generate a random key
fn get_entropy32() -> u32 {
    let next = thread_rng().next_u32();
    println!("Next: {:?}", next);
    //RngCore::try_fill_bytes(&arr).expect("Unable to get entropy");
    return next;
}

fn main() {
    // Get entropy
    let entropy: u32 = get_entropy32();

    // Generate mnemonic from key
/*
    let mnemonic = Mnemonic::from_entropy(&entropy.to_bytes()).expect("Unable to generate entropy");
    println!("Mnemonic is: {:?}", mnemonic.to_string());
    println!(
        "Seed normalised: {:?}",
        hex::encode(mnemonic.to_seed_normalized(&mnemonic.to_string()))
    );
    */
}
