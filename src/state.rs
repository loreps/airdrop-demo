// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use airdrop_demo::AirDropId;
use linera_sdk::views::{linera_views, RootView, SetView, ViewStorageContext};

/// The application state.
#[derive(RootView, async_graphql::SimpleObject)]
#[view(context = "ViewStorageContext")]
pub struct Application {
    pub handled_airdrops: SetView<AirDropId>,
}
