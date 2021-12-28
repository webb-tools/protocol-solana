use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;
use arkworks_utils::utils::bn254_x5_3::{
    FULL_ROUNDS,
    PARTIAL_ROUNDS,
    WIDTH,
    SBOX,
};

pub struct CircomPoseidonHasher;

impl CircomPoseidonHasher {
    pub fn hash(input: &[u8], round_consts: [[u8; 32]; 195], mds_matrix: [[[u8; 32]; 3]; 3]) -> Result<Vec<u8>, Error> {
		let round_consts = round_consts.iter().map(|x| ark_bn254::Fr::from_be_bytes_mod_order(x)).collect::<Vec<_>>();
		let mds_matrix = mds_matrix
			.iter()
			.map(|x| x.iter().map(|y| ark_bn254::Fr::from_be_bytes_mod_order(y)).collect())
			.collect::<Vec<Vec<ark_bn254::Fr>>>();
        let params = PoseidonParameters::<ark_bn254::Fr>::new(
			round_consts,
			mds_matrix,
			FULL_ROUNDS,
			PARTIAL_ROUNDS,
			WIDTH,
			SBOX
		);
        let output: ark_bn254::Fr = <CRH<ark_bn254::Fr> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher;
