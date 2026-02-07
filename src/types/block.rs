struct Block {
 pub Header: BlockHeader,
 pub transactions: Vec<Transactions>,
}


impl Block { 
    pub fn new (Header: BlockHeader,
 transactions:Vec<Transactions>,
 )->Block{
    Block{
        Header,
        transactions,
    }
 }
 /// Create a new block with automatic merkle root calculation
  fn create(version:u32,timestamp:u32,
   prev_hash:[u8;32],
   bits:u32,
   transactions:Vec<Transactions>
  )->Block {
let tx_hashes=transaction.iter().map(|tx|tx.txid()).collected();
 let markel_root=compute_markel_root(&tx_hashes);
let BlockHeader = BlockHeader::new(version,prev_hash,markel_root
                                    ,timestamp,bits);
       Block{Block{BlockHeader,transactions}}                             
  }

 fn hash(&self)->[u8;32]{
    self.Header.hash()
 }

 fn hex_hash(&self)->string{
   self.Header.hex_hash()
 }
fn mine(&mut self,max_nonce:u32)->Option<u32>{
   self.Header.mine(max_nonce)
}

 fn validate (&self)->Result<()>{
   if self.transaction.is_empty(){
      return Err(BitcoinError::ValidationError(
         "First transaction must be coinbased".to_string(),))
   }

   if ! self.transaction[0].is_coinbase(){
      return Err(BitcoinError::ValidationError(
         "First transaction must be coinbase".to_string(),
      ))
   }
   for tx in &self.transaction[1..]{
      if  tx.is_coinbase(){
         return Err(Bitcoin::ValidationError(
            "Non-first transaction cannot be coinbase".to_string(),))
      }
   }
    // Validate merkle root
        let tx_hashes: Vec<[u8; 32]> = self.transactions
            .iter()
            .map(|tx| tx.txid())
            .collect();
        
        let calculated_merkle_root = compute_merkle_root(&tx_hashes);
        if calculated_merkle_root != self.header.merkle_root {
            return Err(BitcoinError::ValidationError(
                "Invalid merkle root".to_string(),
            ));
        }
        // Validate proof of work
        if !self.header.meets_target() {
            return Err(BitcoinError::ValidationError(
                "Block does not meet difficulty target".to_string(),
            ));
        }

        for tx in &self.transaction{
tx.validate()?;
        }
Ok(())
 }
 
 
 
 fn current_height(&self)->u32{
    self.Header.height()
   }

 fn add_block(&mut self,block:Block){
    self.Header=block.Header;
    self.transactions=block.transactions;
    self.Difficulty=block.Difficulty;
 }

 fn validate_block(&self)->bool{
    self.Header.validate()
 }


}