// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use linera_sdk::{
    abis::fungible::FungibleTokenAbi,
    base::{ApplicationId, ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

pub struct ApplicationAbi;

impl ContractAbi for ApplicationAbi {
    type Operation = ();
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
