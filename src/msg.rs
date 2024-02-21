use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub set_zkeys_price: Option<Coin>,
    pub publish_proof_price: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Zkeys {
        public_signal: String,
        vk_alpha1: Uint128,
        vk_beta_1: Uint128,
        vk_gamma_1: Uint128,
    },
    Proof {
        difficuty_issuer: String,
        proof_a: Uint128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ZkeysResponse)]
    IssuerZkeys { address: String },
    #[returns(ProofResponse)]
    ProofResult {
        issuer_address: String,
        prover_address: String,
    },
}

#[cw_serde]
pub struct ConfigResponse {
    pub zkeys_price: Option<Coin>,
    pub proof_price: Option<Coin>,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            zkeys_price: config.zkeys_price,
            proof_price: config.proof_price,
        }
    }
}

#[cw_serde]
pub struct ProofResponse {
    pub proof_a: Uint128,
    pub is_valid: bool,
}

#[cw_serde]
pub struct ZkeysResponse {
    pub public_signal: String,
    pub vk_alpha1: Uint128,
    pub vk_beta_1: Uint128,
    pub vk_gamma_1: Uint128,
}
