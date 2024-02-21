use super::error::ContractError;
use crate::state::{ProofStr, VkeyStr};
use bls12_381::{G1Affine, G2Affine};
use cosmwasm_std::ensure;
use pairing::Engine;
use test_verifier::{Proof, VerifyingKey};

/// convert the proof into the affine type, which will be used to verify
pub fn parse_proof(pof: ProofStr) -> Result<Proof, ContractError> {
    let pi_a = pof.pi_a;

    Ok(Proof { a: pi_a })
}

/// convert the verification key into the affine type, which will be used in verification
pub fn parse_vkey(vk: VkeyStr) -> Result<VerifyingKey, ContractError> {
    let vk_alpha_1 = vk.alpha_1;
    let vk_beta_1 = vk.beta_1;
    let vk_gamma_1 = vk.gamma_1;

    // return verification key
    Ok(VerifyingKey {
        alpha_g1: vk_alpha_1,
        beta_g1: vk_beta_1,
        gamma_g1: vk_gamma_1,
    })
}
