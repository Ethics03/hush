use anyhow::{anyhow, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};

//encrypting here with key and data generating a nonce
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)?;
    let nonce_bytes = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce_bytes, data.as_ref())?;

    Ok((ciphertext, nonce_bytes.to_vec()))
}

//decrypting with same key + nonce
pub fn decrypt(ciphertext: &[u8], key: &[u8], nonce_bytes: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new_from_slice(key)
        .map_err(|_| anyhow!("Invalid key length - must be 32 bytes"))?;

    let nonce_array: &[u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| anyhow!("Invalid nonce length, must be 12 bytes"))?;

    let nonce = Nonce::from(*nonce_array);

    let plaintext = cipher
        .decrypt(&nonce, ciphertext.as_ref())
        .map_err(|_| anyhow!("Decryption failed - wrong password or corrupted data"))?;

    Ok(plaintext)
}
