struct Blockchain{
    pub Block: Vec<Block>,
    pub Difficulty :u32,
}

impl Blockchain {
 pub   fn new()->self{
    let genesis=Block::genesis();
    self{
        Block:vec![genesis],
        difficulty_bits:u32
    }
 }

 pub fn latest_block(&self)->&block{
self.Block.last().unwrap()
 }
   pub fn height (&self)->u32{
self.BLock.len()as u32-1
   }

   pub fn add_block(){
    let hash=self.latest_block().hash();
    if BlockHeader.prev_hash() != hash{
        return Err(BitcoinError::ValidationError(
            "Previous block hash mismatch"),to_string(),)
    }
    Block.validate()?;

    self.Block.push(Block);
    Ok(())
   }

   pub fn validate(&self)->Result<()>{
    for(i,block) in self.Block.iter().enumerate(){
Block.validate()?;
    }
    if i>0{
        let prev_hash=self.blocks[i-1].hash();
        if Block.Header.prev_hash1=prev_hash{
return Err (BitcoinError::ValidationError(
                        format!("Block {} has invalid previous hash", i)))
        }
    }
    Ok(())
   }
}

impl Default for Blockchain{
    fn default()->self{
        Self::new()
    }
}