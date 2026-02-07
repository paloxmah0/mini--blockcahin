struct Tx{
    pub txin:Vec<Txin>,
    pub txou:Vec<Txou>,
}

struct Txin{
    pub prev_hash:String,
    pub public_key:String,
    pub amount:u64,
    pub signature:String,
}

struct Txou{
    pub amount:u64,
    pub public_key:String,
    pub signature:String,
    pub id:uUid,
}

impl Tx{
pub fn new(txin:Vec<Txin>,txou:Vec<Txou>)->Tx{
    Tx{
        txin,
        txou,
    }
  }
pub fn hash(&self)->hash{
    self.calculate_hash(&self);
}

}