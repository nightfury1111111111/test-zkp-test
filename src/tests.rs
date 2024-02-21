#[cfg(test)]
mod test_module {
    use bls12_381::{G1Affine, G2Affine};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary, Coin, Deps, DepsMut, Uint128};

    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, ProofResponse, QueryMsg, ZkeysResponse};
    use crate::state::Config;

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn mock_init_no_price(deps: DepsMut) {
        let msg = InstantiateMsg {
            set_zkeys_price: None,
            publish_proof_price: None,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    fn mock_init_with_price(deps: DepsMut, zkeys_price: Coin, proof_price: Coin) {
        let msg = InstantiateMsg {
            set_zkeys_price: Some(zkeys_price),
            publish_proof_price: Some(proof_price),
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
            .expect("contract successfully handles InstantiateMsg");
    }

    #[test]
    fn proper_init_no_fees() {
        let mut deps = mock_dependencies();

        mock_init_no_price(deps.as_mut());
        assert_config_state(
            deps.as_ref(),
            Config {
                zkeys_price: None,
                proof_price: None,
            },
        );
    }

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        mock_init_with_price(deps.as_mut(), coin(3, "token"), coin(4, "token"));

        assert_config_state(
            deps.as_ref(),
            Config {
                zkeys_price: Some(coin(3, "token")),
                proof_price: Some(coin(4, "token")),
            },
        );
    }

    #[test]
    fn fail_set_zkeys_insufficient_fees() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(2, "token"), coin(2, "token"));
        let info = mock_info("alice_key", &[]);
        let msg = ExecuteMsg::Zkeys {
            public_signal: "33".to_string(),
            vk_alpha1: Uint128::new(7),
            vk_beta_1: Uint128::new(11),
            vk_gamma_1: Uint128::new(2),
        };
        println!("sdfsdfsdsf, {:?}", G1Affine::identity());
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Ok(_) => panic!("set zkeys should fail with insufficient fees"),
            Err(ContractError::InsufficientFundsSend {}) => {}
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    fn mock_alice_set_zkeys(deps: DepsMut, sent: &[Coin]) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::Zkeys {
            public_signal: "33".to_string(),
            vk_alpha1: Uint128::new(7),
            vk_beta_1: Uint128::new(11),
            vk_gamma_1: Uint128::new(3),
        };

        let _res =
            execute(deps, mock_env(), info, msg).expect("contract handles set zkeys parameters");
    }

    fn mock_bob_publish_proof_to_verify(deps: DepsMut, sent: &[Coin]) {
        let info = mock_info("bob_key", sent);
        let msg = ExecuteMsg::Proof {
            difficuty_issuer: "alice_key".to_string(),
            proof_a: Uint128::new(231),
        };

        let _res =
            execute(deps, mock_env(), info, msg).expect("contract handles verify proof failed");
    }

    fn query_zkeys(deps: Deps) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::IssuerZkeys {
                address: "alice_key".to_string(),
            },
        )
        .unwrap();

        // get response
        let value: ZkeysResponse = from_binary(&res).unwrap();
        println!("zkey is :{:?}", value);
    }

    fn query_verification_result(deps: Deps) {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::ProofResult {
                issuer_address: "alice_key".to_string(),
                prover_address: "bob_key".to_string(),
            },
        )
        .unwrap();

        let value: ProofResponse = from_binary(&res).unwrap();
        print!("proof info is: {:?}", value);
    }

    #[test]
    fn sey_zkeys_and_query_works() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        mock_alice_set_zkeys(deps.as_mut(), &[]);

        query_zkeys(deps.as_ref());
    }

    #[test]
    fn verify_proof_and_query_works_with_no_price() {
        let mut deps = mock_dependencies();
        mock_init_no_price(deps.as_mut());
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify(deps.as_mut(), &[]);
        query_verification_result(deps.as_ref());
    }

    #[test]
    fn verify_proof_and_query_works_with_price() {
        let mut deps = mock_dependencies();
        mock_init_with_price(deps.as_mut(), coin(1, "token"), coin(1, "token"));
        // alice set issue difficulty
        mock_alice_set_zkeys(deps.as_mut(), &[coin(2, "token")]);

        // verify the proof of bob
        mock_bob_publish_proof_to_verify(deps.as_mut(), &[coin(2, "token")]);
        query_verification_result(deps.as_ref());
    }
}
