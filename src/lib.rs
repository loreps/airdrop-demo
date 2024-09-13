// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{
    abis::fungible::{Account, FungibleTokenAbi},
    base::{ApplicationId, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = AirDropClaim;
    type Response = ();
}

impl ServiceAbi for ApplicationAbi {
    type Query = ();
    type QueryResponse = ();
}

/// The shared parameters that are specified when the application is instantiated.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub token_id: ApplicationId<FungibleTokenAbi>,
}

/// The information necessary to identify an airdrop.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, async_graphql::SimpleObject)]
pub struct AirDropId {
    external_address: Vec<u8>,
}

/// An airdrop claim.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, async_graphql::SimpleObject)]
pub struct AirDropClaim {
    pub id: AirDropId,
    pub destination: Account,
}
