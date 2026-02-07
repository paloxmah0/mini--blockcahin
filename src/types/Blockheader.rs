struct BlockHeader {
    pub version: u32,
    pub timestamp: u32,      // ← Should be u32, not u64
    pub nonce: u32,          // ← Fixed spelling
    pub prev_hash: [u8; 32], // ← Should be bytes, not String
    pub merkle_root: [u8; 32], // ← Should be bytes, not String
    pub bits: u32,
}

impl BlockHeader {
    pub fn new(
        version: u32,
        timestamp: u32,
        nonce: u32,
        prev_hash: [u8; 32],
        merkle_root: [u8; 32],
        bits: u32
    ) -> BlockHeader {
        BlockHeader {
            version,
            timestamp,
            nonce,
            prev_hash,
            merkle_root,
            bits,
        }
    }

    fn serialize(&self) -> [u8; 80] {
        let mut bytes = [0u8; 80];
        
        // CORRECT Bitcoin order:
        bytes[0..4].copy_from_slice(&self.version.to_le_bytes());
        bytes[4..36].copy_from_slice(&self.prev_hash);
        bytes[36..68].copy_from_slice(&self.merkle_root);
        bytes[68..72].copy_from_slice(&self.timestamp.to_le_bytes());
        bytes[72..76].copy_from_slice(&self.bits.to_le_bytes());
        bytes[76..80].copy_from_slice(&self.nonce.to_le_bytes());
        
        bytes
    }

    fn hash(&self) -> [u8; 32] {
        let serialized = self.serialize();
        double_sha256(&serialized)
    }

    fn hash_hex(&self) -> String {
        let hash_bytes = self.hash();
        let reversed: Vec<u8> = hash_bytes.iter().rev().copied().collect();
        hex::encode(reversed)  // For human-readable display
    }

    fn meet_target(&self, target: &[u8; 32]) -> bool {
        let hash = self.hash();
        hash <= *target  // Array comparison works in Rust!
    }

    fn mine(&mut self, max_nonce: u32) -> Option<u32> {
        for nonce in 0..max_nonce {
            self.nonce = nonce;
            if self.meet_target(&bits_to_target(self.bits)) {
                return Some(nonce);  // Success!
            }
        }
        None  // Failed to find valid nonce
    }
}