use anchor_lang::prelude::msg;
use ark_crypto_primitives::{Error, CRH as CRHTrait};
use ark_ff::{BigInteger, PrimeField};
use ark_std::{marker::PhantomData, vec::Vec};
use arkworks_gadgets::poseidon::CRH;
use arkworks_utils::poseidon::PoseidonParameters;
use arkworks_utils::utils::bn254_x5_3::{
    FULL_ROUNDS,
    PARTIAL_ROUNDS,
    WIDTH,
    SBOX,
    ROUND_CONSTS,
    MDS_ENTRIES, get_poseidon_bn254_x5_3
};
use ark_bn254::Fr as Bn254;
use arkworks_utils::utils::{get_bytes_array_from_hex};

pub struct CircomPoseidonHasher<F: PrimeField>(PhantomData<F>);

fn parse_vec<F: PrimeField>(arr: Vec<&str>) -> Vec<F> {
	let mut res = Vec::new();
    let mut ctr = 0;
	for r in arr.iter() {
        msg!("{}", ctr);
		let c = F::from_be_bytes_mod_order(&get_bytes_array_from_hex(r));
		res.push(c);
        ctr += 1;
	}
	res
}

pub fn parse_matrix<F: PrimeField>(mds_entries: Vec<Vec<&str>>) -> Vec<Vec<F>> {
	let width = mds_entries.len();
	let mut mds: Vec<Vec<F>> = vec![vec![F::zero(); width]; width];
	for i in 0..width {
		for j in 0..width {
			// TODO: Remove unwrap, handle error
			mds[i][j] = F::from_be_bytes_mod_order(&get_bytes_array_from_hex(mds_entries[i][j]));
		}
	}
	mds
}


impl<F: PrimeField> CircomPoseidonHasher<F> {
    pub fn hash(input: &[u8]) -> Result<Vec<u8>, Error> {
        let params = get_poseidon_bn254_x5_3();
        let output: F = <CRH<F> as CRHTrait>::evaluate(&params, input)?;
        let value = output.into_repr().to_bytes_le();
        Ok(value)
    }
}

pub type BN254CircomPoseidon3x5Hasher = CircomPoseidonHasher<Bn254>;
