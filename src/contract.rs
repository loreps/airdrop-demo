// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use airdrop_demo::Parameters;
use linera_sdk::{
    base::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

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
    type Message = ();
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

    async fn execute_operation(&mut self, _operation: Self::Operation) -> Self::Response {}

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
