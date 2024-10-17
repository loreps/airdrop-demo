// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in tests.

use linera_sdk::base::{ApplicationId, BlockHeight, BytecodeId, ChainId, CryptoHash, MessageId};

/// Creates a dummy [`ApplicationId`] to use for testing.
pub fn create_dummy_application_id<Abi>(name: &str, index: u32) -> ApplicationId<Abi> {
    let bytecode_id = BytecodeId::new(
        CryptoHash::test_hash(format!("{name} contract")),
        CryptoHash::test_hash(format!("{name} service")),
    );

    let creation = MessageId {
        chain_id: ChainId(CryptoHash::test_hash("chain")),
        height: BlockHeight::ZERO,
        index,
    };

    ApplicationId {
        bytecode_id,
        creation,
    }
    .with_abi()
}
