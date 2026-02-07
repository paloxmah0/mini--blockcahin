# mini--blockcahin
This is a bitcoin blockchain implemented with rust for learning purpose. 
It contains the following stucture  : 
rust-mini-blockchain/
│
├── README.md
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, demo blockchain run
│   ├── blockchain.rs     # Blockchain logic (blocks, chain, validation)
│   ├── sha256.rs         # SHA256 hashing utilities
│   └── utils.rs          # Helper functions (optional)
└── LICENSE               # MIT License (optional but professional)


## Features
- Create and validate blocks
- SHA256 hashing for block integrity
- Basic transaction structure
- Demonstrates serialization/deserialization with Rust

## Getting Started
1. Install Rust from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Clone the repository:
```bash
git clone https://github.com/<your-username>/rust-mini-blockchain.git
cd rust-mini-blockchain
cargo run

---

### **Step 4: Sample Code for Your Repo**

**src/main.rs**
```rust
mod blockchain;
mod sha256;
mod utils;

use blockchain::{Blockchain, Block};

fn main() {
    let mut chain = Blockchain::new();

    chain.add_block("First block data".to_string());
    chain.add_block("Second block data".to_string());

    for block in chain.blocks.iter() {
        println!("{:?}", block);
    }
}

use crate::sha256::sha256;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut bc = Blockchain { blocks: vec![] };
        let genesis = Block {
            index: 0,
            data: "Genesis Block".to_string(),
            previous_hash: String::from("0"),
            hash: sha256("Genesis Block"),
        };
        bc.blocks.push(genesis);
        bc
    }

    pub fn add_block(&mut self, data: String) {
        let last_hash = self.blocks.last().unwrap().hash.clone();
        let block = Block {
            index: self.blocks.len() as u32,
            data: data.clone(),
            previous_hash: last_hash,
            hash: sha256(&data),
        };
        self.blocks.push(block);
    }
}
use sha2::{Sha256, Digest};

pub fn sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result) }

