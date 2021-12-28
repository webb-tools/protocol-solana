use super::*;
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

use crate::hashing_params::*;

pub struct CircomPoseidonHasher;

impl CircomPoseidonHasher {
    pub fn hash(input: &[u8]) -> Result<Vec<u8>, Error> {
		msg!("Hashing...");
		let mut ctr = 0;
		let round_consts = BN254_X5_3_ROUND_CONSTS_BOOLS.iter().map(|x| {
			msg!("{}", ctr);
			ctr += 1;
			ark_bn254::Fr::from_repr(BigInteger::from_bits_be(&x[..])).unwrap()
		}).collect::<Vec<_>>();
		let mds_matrix = BN254_X5_3_MDS_MATRIX_BOOLS
			.iter()
			.map(|x| x.iter().map(|y| ark_bn254::Fr::from_repr(BigInteger::from_bits_be(&y[..])).unwrap()).collect())
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
