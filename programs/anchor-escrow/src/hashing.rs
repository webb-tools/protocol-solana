use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;
use arkworks_utils::poseidon::fixed_sized_bn254_x5_3_params::FixedPoseidonBN254Parameters;

pub struct CircomPoseidonHasher;

impl CircomPoseidonHasher {
    pub fn hash(input: &[u8], fixed_params: FixedPoseidonBN254Parameters) -> Result<Vec<u8>, Error> {
		let round_consts = fixed_params.round_keys.to_vec();
		let mds_matrix = fixed_params.mds_matrix
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<Vec<ark_bn254::Fr>>>();
        let params = PoseidonParameters::<ark_bn254::Fr>::new(
			round_consts,
			mds_matrix,
			fixed_params.full_rounds,
			fixed_params.partial_rounds,
			fixed_params.width,
			fixed_params.sbox,
		);
        let output: ark_bn254::Fr = <CRH<ark_bn254::Fr> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher;
