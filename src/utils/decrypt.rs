use openssl::{symm::{Cipher, Crypter, Mode}};
use base64;

pub fn decrypt_message(encrypted_key: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Base64 decode the message
    let decoded_message = base64::decode(message)?;

    // Step 2: Extract the IV and the new ciphertext
    let iv = &decoded_message[..16];
    let new_ciphertext = &decoded_message[16..];

    // Step 3: Base64 decode the new ciphertext using openssl's decode_block
    let data_to_decrypt = base64::decode(new_ciphertext)?;

    // Step 4: Append null bytes to the encrypt key until its length is 32
    let mut key = Vec::from(encrypted_key.as_bytes());
    while key.len() < 32 {
        key.push(0);
    }

    // Step 5: Use AES-256-CBC to decrypt the data
    let cipher = Cipher::aes_256_cbc();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, &key, Some(iv))?;
    let mut decrypted_data = vec![0; data_to_decrypt.len() + cipher.block_size()];
    let count = decrypter.update(&data_to_decrypt, &mut decrypted_data)?;
    let rest = decrypter.finalize(&mut decrypted_data[count..])?;
    decrypted_data.truncate(count + rest);
    let decrypted_message = String::from_utf8(decrypted_data)?;

    Ok(decrypted_message)
}
