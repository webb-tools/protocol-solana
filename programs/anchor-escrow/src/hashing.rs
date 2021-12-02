use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::{
    circom::CircomCRH, sbox::PoseidonSbox, PoseidonParameters, Rounds,
};

#[derive(Default, Clone, Copy)]
pub struct PoseidonRounds3x5;

impl Rounds for PoseidonRounds3x5 {
    const FULL_ROUNDS: usize = 8;
    const PARTIAL_ROUNDS: usize = 57;
    const SBOX: PoseidonSbox = PoseidonSbox::Exponentiation(5);
    const WIDTH: usize = 3;
}

pub struct CircomPoseidonHasher<F: PrimeField, P: Rounds>(PhantomData<F>, PhantomData<P>);

impl<F: PrimeField, P: Rounds> CircomPoseidonHasher<F, P> {
    pub fn hash(input: &[u8], param_bytes: &[u8]) -> Result<Vec<u8>, Error> {
        let params = PoseidonParameters::<F>::from_bytes(param_bytes)?;
        let output: F = <CircomCRH<F, P> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

use ark_bn254::Fr as Bn254;
pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher<Bn254, PoseidonRounds3x5>;
