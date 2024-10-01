// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use airdrop_demo::{AirDropClaim, AirDropId};
use alloy_primitives::Address;
use linera_sdk::{
    abis::fungible,
    base::{AccountOwner, ChainId, CryptoHash, Owner},
    bcs, serde_json,
    util::BlockingWait,
    Service,
};

use super::ApplicationService;

/// Tests if a GraphQL mutation can be used to create an [`AirDropClaim`] operation.
#[test]
fn mutation_generates_air_drop_claim() {
    let service = ApplicationService;

    let chain_id = ChainId(CryptoHash::test_hash("chain ID"));
    let claimer = AccountOwner::User(Owner(CryptoHash::test_hash("claimer")));
    let address = Address::random();

    let json_query = format!(
        "{{ \"query\":
            \"mutation {{ \
                airDropClaim( \
                    id: \\\"{address:?}\\\", \
                    destination: {{ \
                        chainId: \\\"{chain_id}\\\", \
                        owner: \\\"{claimer}\\\" \
                    }} \
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

    let operation = bcs::from_bytes::<AirDropClaim>(&serialized_operation)
        .expect("Failed to deserialize returned operation");

    let expected_operation = AirDropClaim {
        id: AirDropId::from(address),
        destination: fungible::Account {
            chain_id,
            owner: claimer,
        },
    };

    assert_eq!(operation, expected_operation);
}
