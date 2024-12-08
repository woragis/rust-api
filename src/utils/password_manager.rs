use aes::Aes256;
use base64::{engine::general_purpose, Engine};
use cbc::{Decryptor, Encryptor};
use cipher::{generic_array::GenericArray, BlockDecryptMut, BlockSizeUser, KeyIvInit};
use rand::Rng;

pub fn generate_key() -> [u8; 32] {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 32];
    rng.fill(&mut key);
    key
}

pub fn encrypt(data: &str, key: &[u8; 32]) -> (String, Vec<u8>) {
    // Generate a random IV (Initialization Vector)
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let encryptor = Encryptor::<Aes256>::new(key.into(), &iv.into());

    // Calculate padding
    let block_size = Aes256::block_size();
    let mut buffer = data.as_bytes().to_vec();

    // Apply PKCS7 padding manually
    let padding_len = block_size - (buffer.len() % block_size);
    buffer.extend(vec![padding_len as u8; padding_len]); // Extend the buffer with padding bytes

    // Convert the buffer into a vector of GenericArray blocks
    let blocks: Vec<GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>> = buffer
        .chunks_exact(block_size)
        .map(GenericArray::clone_from_slice)
        .collect();
    // Encrypt the blocks
    let encrypted_blocks = blocks;
    // Flatten the encrypted blocks back into a single Vec<u8>
    let encrypted_buffer: Vec<u8> = encrypted_blocks
        .iter()
        .flat_map(|block| block.as_slice())
        .cloned()
        .collect();

    // Return base64-encoded ciphertext and the IV
    (
        general_purpose::STANDARD.encode(&encrypted_buffer),
        iv.to_vec(),
    )
}

pub fn decrypt(encrypted_data: &str, key: &[u8; 32], iv: &Vec<u8>) -> String {
    // Decode the base64-encoded ciphertext
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_data)
        .expect("Base64 decode failed");

    // Convert the IV (Vec<u8>) into a GenericArray
    let iv_array = GenericArray::from_slice(&iv);

    // Set up the AES decryptor with the same key and IV
    let mut decryptor = Decryptor::<Aes256>::new(GenericArray::from_slice(key), iv_array);

    // Convert the encrypted data into blocks
    let block_size = Aes256::block_size();
    let blocks: Vec<GenericArray<u8, <Aes256 as BlockSizeUser>::BlockSize>> = encrypted_data
        .chunks_exact(block_size)
        .map(GenericArray::clone_from_slice)
        .collect();

    // Decrypt the blocks
    let mut decrypted_blocks = blocks.clone(); // Clone because blocks will be mutated
    decryptor.decrypt_blocks_mut(&mut decrypted_blocks);

    // Flatten the decrypted blocks back into a single Vec<u8>
    let decrypted_buffer: Vec<u8> = decrypted_blocks
        .iter()
        .flat_map(|block| block.as_slice())
        .cloned()
        .collect();

    // Remove padding (PKCS7)
    let padding_len = *decrypted_buffer.last().unwrap() as usize;
    let decrypted_data = &decrypted_buffer[..decrypted_buffer.len() - padding_len];

    // Convert the decrypted data back into a string
    String::from_utf8(decrypted_data.to_vec()).expect("Decryption failed")
}
