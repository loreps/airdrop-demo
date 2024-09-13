// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use airdrop_demo::{AirDropClaim, AirDropId, Parameters};
use linera_sdk::{
    abis::fungible::{self, Account},
    base::{AccountOwner, Amount, WithContractAbi},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use serde::{Deserialize, Serialize};

use self::state::Application;

pub struct ApplicationContract {
    state: Application,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(ApplicationContract);

impl WithContractAbi for ApplicationContract {
    type Abi = airdrop_demo::ApplicationAbi;
}

impl Contract for ApplicationContract {
    type Message = ApprovedAirDrop;
    type Parameters = Parameters;
    type InstantiationArgument = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = Application::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        ApplicationContract { state, runtime }
    }

    /// Instantiates the application.
    ///
    /// Fails if the [`Parameters`] specified to create the application are invalid.
    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Fetch the parameters to check that they can be successfully deserialized.
        let _parameters = self.runtime.application_parameters();
    }

    /// Verifies an [`AirDropClaim`][`zk_airdrop_demo::AirDropClaim`] and if approved, sends a
    /// message to the application's creator chain to ask the tokens to be delivered.
    async fn execute_operation(&mut self, claim: Self::Operation) -> Self::Response {}

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl ApplicationContract {
    /// Calculates the [`Amount`] to be airdropped for one [`AirDropClaim`].
    async fn airdrop_amount(&mut self, _claim: &AirDropClaim) -> Amount {
        Amount::ONE
    }
}

/// An airdrop claim that has been approved and sent back to the creator chain to deliver the
/// tokens.
#[derive(Debug, Deserialize, Serialize)]
pub struct ApprovedAirDrop {
    id: AirDropId,
    amount: Amount,
    destination: Account,
}
