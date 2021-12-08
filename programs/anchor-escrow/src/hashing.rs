use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;

pub struct CircomPoseidonHasher<F: PrimeField>(PhantomData<F>);

impl<F: PrimeField> CircomPoseidonHasher<F> {
    pub fn hash(input: &[u8], param_bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let params = PoseidonParameters::<F>::from_bytes(param_bytes)?;
        let output: F = <CRH<F> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

use ark_bn254::Fr as Bn254;
pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher<Bn254>;
