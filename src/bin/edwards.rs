use ed25519_dalek::{
    pkcs8::{self, EncodePrivateKey, EncodePublicKey},
    Signer, Verifier, SigningKey,
};
use rand::rngs::OsRng;
use std::fmt;

#[derive(Debug)]
enum CustomError {
    DalekPkcs8SpkiError(ed25519_dalek::pkcs8::spki::Error),
    DalekPkcs8Error(ed25519_dalek::pkcs8::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::DalekPkcs8SpkiError(err) => write!(f, "DalekPkcs8SpkiError: {:?}", err),
            CustomError::DalekPkcs8Error(err) => write!(f, "DalekPkcs8Error: {:?}", err),

        }
    }
}

impl std::error::Error for CustomError {}

fn main() -> Result<(), CustomError> {
    
    // Use a cryptographically secure pseudo random number generator
    let mut rng = OsRng;

    // Generate ED25519 keypair 
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    // Print keypair in binary DER format
    println!("\n\nPKCS#8 private key DER is: {:?}\n", signing_key.to_pkcs8_der().map_err(CustomError::DalekPkcs8Error)?.to_bytes());
    println!("PKCS#8 public key DER is: {:?}\n\n", verifying_key.to_public_key_der().map_err(CustomError::DalekPkcs8SpkiError)?);

    // Print keypair in text PEM format
    println!("PKCS#8 private key PEM is: {:?}\n", signing_key.to_pkcs8_pem(pkcs8::spki::der::pem::LineEnding::CRLF).map_err(CustomError::DalekPkcs8Error)?);
    println!("PKCS#8 public key PEM is: {:?}\n", verifying_key.to_public_key_pem(pkcs8::spki::der::pem::LineEnding::CRLF).map_err(CustomError::DalekPkcs8SpkiError)?);

    // Sign a message with the signing key
    let message: &[u8] = b"This is a test message for rust dalek";
    let signature = signing_key.sign(message);

    // Verify message with the verifying key
    assert!(verifying_key.verify(message, &signature).is_ok());

    Ok(())
}
