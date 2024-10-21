// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

#[cfg(test)]
mod service_unit_tests;
mod state;

use std::sync::{Arc, Mutex};

use airdrop_demo::{AirDropClaim, Parameters};
use async_graphql::{EmptySubscription, Schema};
use linera_sdk::{
    abis::fungible, base::WithServiceAbi, bcs, ensure, http, serde_json, Service, ServiceRuntime,
};

#[derive(Clone)]
pub struct ApplicationService {
    runtime: Arc<Mutex<ServiceRuntime<Self>>>,
}

linera_sdk::service!(ApplicationService);

impl WithServiceAbi for ApplicationService {
    type Abi = airdrop_demo::ApplicationAbi;
}

impl Service for ApplicationService {
    type Parameters = Parameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        ApplicationService {
            runtime: Arc::new(Mutex::new(runtime)),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(Query(self.clone()), Mutation, EmptySubscription)
            .finish()
            .execute(query)
            .await
    }
}

/// Root type that defines all the GraphQL queries available from the service.
pub struct Query(ApplicationService);

#[async_graphql::Object]
impl Query {
    /// Checks if an address is eligible to claim an airdrop.
    async fn check_eligibility(
        &self,
        address: String,
        api_token: String,
    ) -> async_graphql::Result<bool> {
        let lowercase_address = address.to_lowercase();

        let query = format!(
            "{{ \"sqlText\": \"\
                SELECT COUNT(*) FROM (\
                    SELECT * FROM ETHEREUM.NATIVE_WALLETS \
                    WHERE WALLET_ADDRESS = '{lowercase_address}' AND BALANCE > 0 \
                    LIMIT 1\
                );\
            \" }}"
        );

        let mut runtime = self
            .0
            .runtime
            .lock()
            .expect("Panics should abort service, so mutex should never be poisoned");

        let response = runtime.http_request(
            http::Request::post(SXT_GATEWAY_URL, query.as_bytes())
                .with_header("Content-Type", b"application/json")
                .with_header("Authorization", format!("Bearer {api_token}").as_bytes()),
        );

        ensure!(
            response.status == 200,
            async_graphql::Error::new(format!(
                "Failed to perform Space-and-Time query. Status-code: {}",
                response.status
            ))
        );

        let result = serde_json::from_slice::<Vec<serde_json::Map<String, serde_json::Value>>>(
            &response.body,
        )
        .map_err(|_| async_graphql::Error::new("Invalid response from Space-and-Time Gateway"))?;

        ensure!(
            result.len() == 1,
            async_graphql::Error::new(format!(
                "Expected a single query result from Space-and-Time, got {}",
                result.len()
            ))
        );
        ensure!(
            result[0].len() == 1,
            async_graphql::Error::new(format!(
                "Expected a single result column from Space-and-Time query, got {}",
                result[0].len()
            ))
        );

        let query_result = result[0]["COUNT(1)"]
            .as_i64()
            .ok_or_else(|| async_graphql::Error::new("Query result is not a single integer"))?;

        Ok(query_result > 0)
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
            api_token,
        })
        .expect("`AirDropClaim` should be serializable"))
    }
}

/// The URL of the Space-and-Time Gateway API.
const SXT_GATEWAY_URL: &str = "https://api.spaceandtime.dev/v1/sql";
