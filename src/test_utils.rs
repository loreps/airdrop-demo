// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Helper functions used in tests.

use alloy_primitives::Signature;
use alloy_sol_types::SolStruct;
use k256::ecdsa::SigningKey;
use linera_sdk::{
    abis::fungible,
    base::{ApplicationId, BlockHeight, BytecodeId, ChainId, CryptoHash, MessageId},
};

use crate::{
    signature_payload::{self, AIRDROP_CLAIM_DOMAIN},
    ApplicationAbi,
};

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

/// Creates a [`Signature`] for an airdrop claim.
pub fn sign_claim(
    signer: &SigningKey,
    application_id: ApplicationId<ApplicationAbi>,
    claimer: fungible::Account,
) -> Signature {
    let payload = signature_payload::AirDropClaim::new(application_id, &claimer);

    let hash = payload.eip712_signing_hash(&AIRDROP_CLAIM_DOMAIN);

    signer
        .sign_prehash_recoverable(hash.as_slice())
        .expect("Payload hash should be signable with `SigningKey`")
        .into()
}
