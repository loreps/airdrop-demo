// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(test)]
mod service_unit_tests;
mod state;

use airdrop_demo::{AirDropClaim, AirDropId, Parameters};
use async_graphql::{connection::EmptyFields, EmptySubscription, Schema};
use linera_sdk::{abis::fungible, base::WithServiceAbi, bcs, Service, ServiceRuntime};

#[derive(Clone)]
pub struct ApplicationService;

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = airdrop_demo::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = Parameters;

    async fn new(_runtime: ServiceRuntime<Self>) -> Self {
        ApplicationService
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(EmptyFields, Mutation, EmptySubscription)
            .finish()
            .execute(query)
            .await
    }
}

/// Root type that defines all the GraphQL mutations available from the service.
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    /// Claims an airdrop.
    async fn air_drop_claim(&self, id: AirDropId, destination: fungible::Account) -> Vec<u8> {
        let signature = "0x0000000000000000000000000000000000000000000000000000000000000000\
            000000000000000000000000000000000000000000000000000000000000000000"
            .parse()
            .expect("Dummy signature is invalid");

        bcs::to_bytes(&AirDropClaim {
            id,
            signature,
            destination,
        })
        .expect("`AirDropClaim` should be serializable")
    }
}
