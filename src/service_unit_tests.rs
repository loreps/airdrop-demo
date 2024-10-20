// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::sync::{Arc, Mutex};

use airdrop_demo::{
    test_utils::{create_dummy_application_id, sign_claim},
    AirDropClaim,
};
use k256::ecdsa::SigningKey;
use linera_sdk::{
    abis::fungible,
    base::{AccountOwner, ChainId, CryptoHash, Owner},
    bcs, serde_json,
    service::MockServiceRuntime,
    util::BlockingWait,
    Service,
};
use rand::rngs::OsRng;

use super::ApplicationService;

/// Tests if a GraphQL mutation can be used to create an [`AirDropClaim`] operation.
#[test]
fn mutation_generates_air_drop_claim() {
    let service = create_service();

    let chain_id = ChainId(CryptoHash::test_hash("chain ID"));
    let claimer = AccountOwner::User(Owner(CryptoHash::test_hash("claimer")));
    let destination = fungible::Account {
        chain_id,
        owner: claimer,
    };

    let api_token = "API token".to_owned();
    let application_id = create_dummy_application_id("zk-airdrop", 1);
    let signing_key = SigningKey::random(&mut OsRng);
    let signature = sign_claim(&signing_key, application_id, destination);
    let signature_string = hex::encode(signature.as_bytes());

    let json_query = format!(
        "{{ \"query\":
            \"mutation {{ \
                airDropClaim( \
                    signature: \\\"{signature_string}\\\", \
                    destination: {{ \
                        chainId: \\\"{chain_id}\\\", \
                        owner: \\\"{claimer}\\\" \
                    }}, \
                    apiToken: \\\"{api_token}\\\" \
                ) \
            }}\"
        }}"
    );

    let query = serde_json::from_str(&json_query).expect("Failed to deserialize GraphQL claim");

    let response = service.handle_query(query).blocking_wait();

    let async_graphql::Value::Object(response_object) = response.data else {
        panic!("Unexpected response data from query: {response:?}");
    };
    let async_graphql::Value::List(ref claim_bytes) = response_object["airDropClaim"] else {
        panic!("Missing serialized `airDropClaim` in response object");
    };

    let serialized_operation = claim_bytes
        .iter()
        .map(|wrapped_byte| {
            let async_graphql::Value::Number(byte_value) = wrapped_byte else {
                panic!("Serialized `airDropClaim` is not a list of numbers");
            };
            let byte_integer = byte_value
                .as_u64()
                .expect("Serialized `airDropClaim` is not a list of integers");

            u8::try_from(byte_integer).expect("Serialized `airDropClaim` is not a list of bytes")
        })
        .collect::<Vec<u8>>();

    let mut operation = bcs::from_bytes::<AirDropClaim>(&serialized_operation)
        .expect("Failed to deserialize returned operation");

    operation.signature = operation.signature.with_parity_bool();

    let expected_operation = AirDropClaim {
        signature,
        destination: fungible::Account {
            chain_id,
            owner: claimer,
        },
        api_token,
    };

    assert_eq!(operation, expected_operation);
}

/// Creates an [`ApplicationService`] instance.
fn create_service() -> ApplicationService {
    let runtime = MockServiceRuntime::new();

    ApplicationService {
        runtime: Arc::new(Mutex::new(runtime)),
    }
}
