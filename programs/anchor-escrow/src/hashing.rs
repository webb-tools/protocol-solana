use anchor_lang::prelude::msg;
use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;
use arkworks_utils::utils::bn254_x5_3::{FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH, SBOX, get_mds_poseidon_bn254_x5_3, get_rounds_poseidon_bn254_x5_3};
pub struct CircomPoseidonHasher<F: PrimeField>(PhantomData<F>);

impl<F: PrimeField> CircomPoseidonHasher<F> {
    pub fn hash(input: &[u8]) -> Result<Vec<u8>, Error> {
        msg!("before rounds");
        let rounds = get_rounds_poseidon_bn254_x5_3();
        msg!("before mds");
        let mds = get_mds_poseidon_bn254_x5_3();
        msg!("before params");
        let params = PoseidonParameters::<F>::new(rounds, mds, FULL_ROUNDS, PARTIAL_ROUNDS, WIDTH, SBOX);
        msg!("before output");
        let output: F = <CRH<F> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

use ark_bn254::Fr as Bn254;
pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher<Bn254>;
