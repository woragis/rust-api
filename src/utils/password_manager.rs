use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use rand::Rng;


pub fn generate_key(key_size: usize) -> Vec<u8> {
    match key_size {
        16 | 24 | 32 => (), // Valid sizes for AES
        _ => panic!("Invalid key size! Must be 16, 24, or 32 bytes."),
    }
    let mut rng = rand::thread_rng();
    (0..key_size).map(|_| rng.gen()).collect()
}

fn pad_to_block_size(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_length = block_size - (data.len() % block_size);
    let mut padded = data.to_vec();
    padded.extend(vec![padding_length as u8; padding_length]);
    padded
}

fn remove_padding(data: &[u8]) -> Vec<u8> {
    let padding_length = *data.last().unwrap() as usize;
    data[..data.len() - padding_length].to_vec()
}


pub fn encrypt(key: &[u8], data: &[u8], block_size: usize) -> Vec<u8> {
    let padded_data = pad_to_block_size(data, block_size);
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut encrypted_data = Vec::new();
    for chunk in padded_data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted_data.extend_from_slice(&block);
    }
    encrypted_data
}


pub fn decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut decrypted_data = Vec::new();

    for chunk in data.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(& mut block);
        decrypted_data.extend_from_slice(&block);
    }
    remove_padding(&decrypted_data)
}


pub fn vec_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data).to_string()
}
