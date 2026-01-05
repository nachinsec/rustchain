pub struct Block {
    pub index: u64,
    pub timestamp: u128, // Unix timestamp in milliseconds
    pub previous_hash: String, // next -> [u8; 32] SHA-256 hash
    pub data: String,
    pub hash: String, // next -> [u8; 32], // SHA-256 hash
}

impl Block {
    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str) -> String {
        // Implementation for calculating the hash of the block
        let mut buffer: Vec<u8> = Vec::new();
        
        let index_bytes = index.to_be_bytes();
        buffer.extend(8u32.to_be_bytes()); // length of index in bytes
        buffer.extend(&index_bytes);

        let timestamp_bytes = timestamp.to_be_bytes();
        buffer.extend(16u32.to_be_bytes()); // length of timestamp in bytes
        buffer.extend(&timestamp_bytes);
    
        let previous_hash_bytes = previous_hash.as_bytes();
        buffer.extend((previous_hash_bytes.len() as u32).to_be_bytes()); // length of previous_hash in bytes
        buffer.extend(previous_hash_bytes);

        let data_bytes = data.as_bytes();
        buffer.extend((data_bytes.len() as u32).to_be_bytes()); // length of data in bytes
        buffer.extend(data_bytes);
    }
}