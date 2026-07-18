use rs_merkle::{MerkleProof, MerkleTree as RsMerkleTree, algorithms::Sha256};

pub struct MerkleTree{
    leaves : Vec<[u8;32]>,
    tree   : RsMerkleTree<Sha256>,
}

impl MerkleTree{
    pub fn FromLeaves(leaves:Vec<[u8;32]>)->Self{
        let tree = RsMerkleTree::<Sha256>::from_leaves(&leaves);
        Self{ leaves: leaves, tree: tree }
    }

    pub fn FromTree( tree: RsMerkleTree<Sha256>)->Self{
        let leaves= tree.leaves().clone().unwrap_or(vec![]);
        Self{ leaves: leaves, tree: tree  }
    }

    pub fn GetRoot(&self)->Option<[u8;32]>{
        self.tree.root()
    }

    pub fn BuildProofFor(&self, indices: &[usize])-> MerkleProof<Sha256>{
        self.tree.proof(&indices)
    }

    // update leaves at given indices with new values
    pub fn Transition(&mut self, indices: Vec<usize>, newValues: Vec<[u8;32]>)->MerkleProof<Sha256>{
        let proof = self.BuildProofFor(&indices);
        let total_leaves = self.leaves.len();
        _ = proof
            .root(&indices, &newValues, total_leaves)
            .expect("Failed to calculate new root from proof");
        for i in 0..indices.len() {
            let target_index = indices[i];
            let new_hash = newValues[i];
            self.leaves[target_index] = new_hash;
        }
        self.tree = RsMerkleTree::<Sha256>::from_leaves(&self.leaves);
        proof
    }   
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_merkle::algorithms::Sha256;
    use rs_merkle::Hasher;

    fn leafHash(value: &str) -> [u8; 32] {
        Sha256::hash(value.as_bytes())
    }

    #[test]
    fn testTransitionProofValidation() {
        // 1. Create initial state and track the genesis root
        let initialValues = vec![
            "account-0:100",
            "account-1:100",
            "account-2:100",
            "account-3:100",
            "account-4:100",
        ];

        let initialLeaves: Vec<[u8; 32]> = initialValues.iter().map(|v| leafHash(v)).collect();
        
        let mut merkleTree = MerkleTree::FromLeaves(initialLeaves.clone());
        let genesisRoot = merkleTree.GetRoot().expect("failed to get genesis root");

        // 2. Define the target indices and their upcoming new values
        let initialIndices = vec![1usize, 3, 4];
        let updateIndices =  vec![1usize, 3, 4];
        let oldLeavesToProve = vec![initialLeaves[1],  initialLeaves[3], initialLeaves[4]];
        
        let newValues = vec![
            leafHash("account-1:200"),
            leafHash("account-3:550"),
            leafHash("account-4:550"),
        ];

        let malicious_indices=vec![1usize,3];
        let malicious_excluding= vec![            
            leafHash("account-1:200"),
            leafHash("account-3:550"),
        ];


        // 3. Perform the transition and harvest the proof package
        let totalLeavesCount = initialLeaves.len();
        let proof = merkleTree.Transition(updateIndices.clone(), newValues.clone());
        
        // Track the newly updated root state
        let newRoot = merkleTree.GetRoot().expect("failed to get new root");

        // 4. Verify the proof against the GENESIS state
        let verifiesGenesis = proof.verify(
            genesisRoot, 
            &initialIndices, 
            &oldLeavesToProve, 
            totalLeavesCount
        );
        assert!(verifiesGenesis, "Proof failed to verify the old leaves against genesis root");

        // 5. Verify the EXACT SAME proof against the NEW state
        let verifiesNew = proof.verify(
            newRoot, 
            &updateIndices, 
            &newValues, 
            totalLeavesCount
        );
        assert!(verifiesNew, "Proof failed to verify the new leaves against new root");
        
        let verify_malicious_exclusion=proof.verify(newRoot, &malicious_indices, &malicious_excluding, 2);
        assert_ne!(verify_malicious_exclusion, true, "Proof failed to verify malicious exclusion");
    }
}