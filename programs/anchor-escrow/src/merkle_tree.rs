use super::*;

pub enum MerkleTreeError {
    InvalidRootIndex,
    InvalidLevel,
    InvalidRoot,
    TreeFull,
}

impl MerkleTreeAccount {
    pub fn insert(&mut self, commitment: [u8; 32]) -> Result<u32, MerkleTreeError> {
        let next_index = self.next_index;
        if (next_index == 2u32.pow(self.levels as u32)) {
            return Err(MerkleTreeError::TreeFull);
        }

        let mut curr_index = next_index;
        let mut curr_level_hash = commitment;

        for i in 0..self.levels {
            let (left, right) = if curr_index % 2 == 0 {
                self.filled_subtrees[i as usize] = curr_level_hash;
                (curr_level_hash, self.zeroes(i))
            } else {
                (self.filled_subtrees[i as usize], curr_level_hash)
            };

            curr_level_hash = self.hash_left_right(left, right);
            curr_index = curr_index >> 1;
        }

        let new_root_index = (self.current_root_index + 1) % 32;
        self.current_root_index = new_root_index;
        self.roots[new_root_index as usize] = curr_level_hash;
        self.next_index = next_index + 1;
        Ok(next_index)
        // uint32 _nextIndex = nextIndex;
        // require(_nextIndex != uint32(2)**levels, "Merkle tree is full. No more leaves can be added");
        // uint32 currentIndex = _nextIndex;
        // bytes32 currentLevelHash = _leaf;
        // bytes32 left;
        // bytes32 right;

        // for (uint32 i = 0; i < levels; i++) {
        //   if (currentIndex % 2 == 0) {
        // 	left = currentLevelHash;
        // 	right = zeros(i);
        // 	filledSubtrees[i] = currentLevelHash;
        //   } else {
        // 	left = filledSubtrees[i];
        // 	right = currentLevelHash;
        //   }
        //   currentLevelHash = hashLeftRight(hasher, left, right);
        //   currentIndex /= 2;
        // }

        // uint32 newRootIndex = (currentRootIndex + 1) % ROOT_HISTORY_SIZE;
        // currentRootIndex = newRootIndex;
        // roots[newRootIndex] = currentLevelHash;
        // nextIndex = _nextIndex + 1;
        // return _nextIndex;
    }

    fn hash_left_right(&self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        [0u8; 32]
    }

    fn zeroes(&self, i: u8) -> [u8; 32] {
        let mut zeroes = [0u8; 32];
        zeroes
    }
}
