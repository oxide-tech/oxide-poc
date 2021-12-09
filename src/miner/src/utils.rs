use rand::Rng;

// Generates a random nonce as a u64
pub(crate) fn gen_nonce() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen::<u64>()
}

pub(crate) fn hash_as_u128(hash: &Vec<u8>) -> u128 {
    let mut buffer: [u8; 16] = [0; 16];
    buffer.clone_from_slice(&hash[0..16]);

    return u128::from_ne_bytes(buffer);
}