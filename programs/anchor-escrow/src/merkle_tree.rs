use super::*;

use hashing::*;

pub enum MerkleTreeError {
    InvalidRootIndex,
    InvalidLevel,
    InvalidRoot,
    TreeFull,
}

impl MerkleTreeAccount {
    pub fn insert(&mut self, commitment: [u8; 32]) -> Result<u32, MerkleTreeError> {
        let next_index = self.next_index;
        msg!("next index: {:?}", next_index);
        msg!("levels: {:?}", self.levels);
        msg!("max leaf count: {:?}", 2u32.pow(self.levels as u32));
        if next_index == 2u32.pow(self.levels as u32) {
            return Err(MerkleTreeError::TreeFull);
        }

        let mut curr_index = next_index;
        let mut curr_level_hash = commitment;
        msg!("curr hash: {:?}", curr_level_hash);
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
    }

    fn hash_left_right(&self, left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
        let mut buf32 = [0u8; 32];
        let mut buf = vec![];
        buf.extend_from_slice(&left);
        buf.extend_from_slice(&right);
        println!("buf: {:?}", buf);
        if let Ok(value) = BN254CircomPoseidon3x5Hasher::hash(&self.params, &buf) {
            buf32.copy_from_slice(&value);
        }

        buf32
    }

    fn zeroes(&self, i: u8) -> [u8; 32] {
        match i {
            0 => [
                47, 229, 76, 96, 211, 172, 171, 243, 52, 58, 53, 182, 235, 161, 93, 180, 130, 27,
                52, 15, 118, 231, 65, 226, 36, 150, 133, 237, 72, 153, 175, 108,
            ],
            1 => [
                19, 227, 127, 45, 108, 184, 108, 120, 204, 193, 120, 134, 7, 194, 177, 153, 120,
                140, 107, 176, 166, 21, 162, 31, 46, 122, 142, 136, 56, 66, 34, 248,
            ],
            2 => [
                33, 113, 38, 250, 53, 44, 50, 104, 150, 232, 194, 128, 62, 236, 143, 214, 58, 213,
                12, 246, 94, 223, 239, 39, 164, 26, 158, 50, 220, 98, 39, 101,
            ],
            3 => [
                14, 40, 166, 26, 155, 62, 145, 0, 125, 90, 158, 58, 218, 24, 225, 178, 77, 109, 35,
                12, 97, 131, 136, 238, 93, 243, 76, 172, 215, 57, 126, 238,
            ],
            4 => [
                39, 149, 52, 71, 166, 151, 152, 57, 83, 107, 173, 197, 66, 94, 209, 95, 173, 176,
                226, 146, 233, 188, 54, 249, 47, 10, 165, 207, 165, 1, 53, 135,
            ],
            5 => [
                25, 65, 145, 237, 191, 185, 29, 16, 246, 167, 175, 211, 21, 243, 48, 149, 65, 12,
                120, 1, 196, 113, 117, 194, 223, 109, 194, 204, 224, 227, 175, 252,
            ],
            6 => [
                23, 51, 222, 206, 23, 215, 17, 144, 81, 109, 186, 241, 146, 121, 54, 250, 100, 61,
                199, 7, 159, 192, 204, 115, 29, 233, 214, 132, 90, 71, 116, 31,
            ],
            7 => [
                38, 120, 85, 167, 220, 117, 219, 57, 216, 29, 23, 249, 93, 10, 122, 165, 114, 191,
                90, 225, 159, 77, 176, 232, 66, 33, 210, 178, 239, 153, 146, 25,
            ],
            8 => [
                17, 132, 225, 24, 54, 180, 195, 106, 216, 35, 138, 52, 14, 204, 9, 133, 238, 186,
                102, 83, 39, 227, 62, 155, 14, 54, 65, 2, 124, 39, 98, 13,
            ],
            9 => [
                7, 2, 171, 131, 161, 53, 215, 245, 83, 80, 171, 27, 250, 169, 11, 171, 216, 252,
                29, 43, 62, 106, 114, 21, 56, 26, 123, 34, 19, 214, 197, 206,
            ],
            10 => [
                46, 236, 192, 222, 129, 76, 253, 140, 87, 206, 136, 43, 171, 178, 227, 13, 29, 165,
                102, 33, 174, 247, 164, 127, 50, 145, 207, 254, 174, 194, 106, 215,
            ],
            11 => [
                40, 11, 192, 33, 69, 193, 85, 213, 131, 53, 133, 182, 199, 176, 133, 1, 5, 81, 87,
                221, 48, 206, 0, 83, 25, 98, 29, 196, 98, 211, 59, 71,
            ],
            12 => [
                4, 81, 50, 34, 29, 31, 160, 167, 244, 174, 216, 172, 210, 203, 236, 30, 33, 137,
                183, 115, 44, 203, 46, 194, 114, 185, 198, 15, 13, 90, 252, 91,
            ],
            13 => [
                39, 244, 39, 204, 191, 88, 164, 75, 18, 112, 171, 190, 78, 218, 107, 165, 59, 214,
                172, 77, 136, 207, 30, 0, 161, 60, 67, 113, 206, 113, 211, 102,
            ],
            14 => [
                22, 23, 234, 174, 80, 100, 242, 110, 143, 138, 100, 147, 174, 146, 191, 222, 215,
                253, 231, 27, 101, 223, 28, 166, 213, 220, 236, 13, 247, 11, 44, 239,
            ],
            15 => [
                32, 198, 180, 0, 208, 234, 27, 21, 67, 87, 3, 195, 28, 49, 238, 99, 173, 123, 165,
                200, 218, 102, 206, 194, 121, 111, 234, 206, 165, 117, 171, 202,
            ],
            16 => [
                9, 88, 157, 219, 67, 135, 35, 245, 58, 142, 87, 189, 173, 167, 197, 248, 237, 103,
                232, 254, 206, 56, 137, 167, 54, 24, 115, 41, 101, 100, 94, 236,
            ],
            17 => [
                0, 100, 182, 167, 56, 165, 255, 83, 125, 183, 178, 32, 243, 57, 79, 14, 203, 211,
                91, 253, 53, 92, 84, 37, 220, 17, 102, 191, 50, 54, 7, 155,
            ],
            18 => [
                9, 93, 229, 98, 129, 177, 213, 5, 94, 137, 124, 53, 116, 255, 121, 13, 94, 232, 29,
                188, 93, 247, 132, 173, 45, 103, 121, 94, 85, 124, 158, 159,
            ],
            19 => [
                17, 207, 46, 40, 135, 170, 33, 150, 58, 110, 193, 66, 137, 24, 62, 254, 77, 76, 96,
                241, 78, 205, 61, 111, 224, 190, 235, 223, 133, 90, 155, 99,
            ],
            20 => [
                43, 15, 111, 192, 23, 159, 166, 91, 111, 115, 98, 124, 14, 30, 132, 199, 55, 77,
                46, 174, 196, 76, 154, 72, 242, 87, 19, 147, 234, 119, 188, 187,
            ],
            21 => [
                22, 253, 182, 55, 194, 171, 249, 192, 249, 136, 219, 242, 253, 100, 37, 140, 70,
                251, 106, 39, 61, 83, 123, 44, 241, 96, 62, 164, 96, 177, 50, 121,
            ],
            22 => [
                33, 187, 215, 233, 68, 246, 18, 77, 173, 76, 55, 109, 249, 204, 18, 231, 202, 102,
                228, 125, 255, 112, 63, 247, 206, 219, 26, 69, 78, 220, 240, 255,
            ],
            23 => [
                39, 132, 248, 34, 11, 28, 150, 62, 70, 143, 89, 15, 19, 123, 170, 161, 98, 91, 59,
                146, 162, 122, 217, 182, 232, 78, 176, 211, 69, 77, 153, 98,
            ],
            24 => [
                22, 172, 225, 166, 91, 117, 52, 20, 47, 140, 193, 170, 216, 16, 179, 214, 167, 167,
                76, 169, 5, 217, 194, 117, 203, 152, 186, 87, 229, 9, 252, 16,
            ],
            25 => [
                35, 40, 6, 140, 106, 140, 36, 38, 81, 36, 222, 189, 143, 225, 13, 63, 41, 240, 102,
                94, 167, 37, 166, 94, 54, 56, 246, 25, 42, 150, 160, 19,
            ],
            26 => [
                45, 219, 153, 27, 225, 240, 40, 2, 36, 17, 180, 196, 210, 194, 32, 67, 229, 231,
                81, 193, 32, 115, 111, 0, 173, 245, 74, 202, 177, 201, 172, 20,
            ],
            27 => [
                1, 19, 121, 132, 16, 234, 235, 149, 5, 106, 70, 79, 112, 82, 30, 181, 131, 119,
                192, 21, 95, 47, 229, 24, 165, 89, 77, 56, 204, 32, 156, 192,
            ],
            28 => [
                32, 45, 26, 230, 21, 38, 240, 208, 208, 30, 248, 15, 181, 212, 5, 90, 122, 244, 87,
                33, 2, 76, 44, 36, 207, 253, 106, 55, 152, 245, 77, 80,
            ],
            29 => [
                35, 171, 50, 52, 83, 116, 129, 41, 242, 118, 95, 121, 97, 80, 34, 245, 190, 189,
                111, 64, 150, 167, 150, 48, 10, 171, 4, 154, 96, 176, 241, 135,
            ],
            30 => [
                31, 21, 88, 95, 137, 71, 227, 120, 188, 248, 189, 145, 135, 22, 121, 157, 169, 9,
                172, 219, 148, 76, 87, 21, 11, 30, 180, 86, 95, 218, 138, 160,
            ],
            31 => [
                30, 176, 100, 178, 16, 85, 172, 106, 53, 12, 244, 30, 179, 14, 76, 226, 203, 25,
                104, 2, 23, 223, 58, 36, 54, 23, 194, 131, 129, 133, 173, 6,
            ],
            // Reuse index 0 for wildcard
            _ => [
                47, 229, 76, 96, 211, 172, 171, 243, 52, 58, 53, 182, 235, 161, 93, 180, 130, 27,
                52, 15, 118, 231, 65, 226, 36, 150, 133, 237, 72, 153, 175, 108,
            ],
        }
    }
}
