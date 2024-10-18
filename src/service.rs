// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(test)]
mod service_unit_tests;
mod state;

use airdrop_demo::{AirDropClaim, Parameters};
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
    async fn air_drop_claim(
        &self,
        destination: fungible::Account,
        signature: String,
        api_token: String,
    ) -> async_graphql::Result<Vec<u8>> {
        let signature = signature
            .parse()
            .map_err(|_| async_graphql::Error::new("Signature could not be parsed"))?;

        Ok(bcs::to_bytes(&AirDropClaim {
            signature,
            destination,
        })
        .expect("`AirDropClaim` should be serializable"))
    }
}
