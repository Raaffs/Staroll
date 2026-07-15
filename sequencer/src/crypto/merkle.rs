
pub struct MerkleTree{
    leaves : Vec<[u8;32]>
}

impl MerkleTree{
    pub fn new(leaves:Vec<[u8;32]>)->Self{
        Self{ leaves: leaves }
    }

    pub fn BuildTree(&self){}

    pub fn GetRoot(&self)->[u8;32]{
        [0;32]
    }

    pub fn GetProofFor(&self, leaf: [u8;32], tree: Vec<Vec<[u8;32]>>){
        
    }

    pub fn VerifyProof(leaf: [u8;32], proof: Vec<[u8;32]>, root: [u8;32]){
        
    }

    pub fn GetMultiProof(leaf: [u8;32], proof: Vec<[u8;32]>, root: [u8;32]){
        
    }

    pub fn VerifyMultiProof(leaf: [u8;32], proof: Vec<[u8;32]>, root: [u8;32]){
        
    }
        
}