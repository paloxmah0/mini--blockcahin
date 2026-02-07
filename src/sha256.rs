pub fn sha256(data:&[u8])->[u8;32]{
    let mut hasher = sha256::new();
    hasher.update(data);
    let results = hasher.finalize();
    let mut hash_bytes = [0u8;32];
    hash_bytes.copy_from_slice(&results);
    hash_bytes
}

pub fn sha256_hex(data:&[u8])->String{
    let mut hasher=sha256(data);
    byte_to_hex(&hasher)
}