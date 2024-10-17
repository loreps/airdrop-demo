// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use airdrop_demo::{test_utils::create_dummy_application_id, AirDropClaim, AirDropId, Parameters};
use alloy_primitives::Address;
use linera_sdk::{
    abis::fungible::{self, Account, FungibleResponse},
    base::{AccountOwner, Amount, ApplicationId, ChainId, CryptoHash, Destination, Owner},
    bcs,
    util::BlockingWait,
    views::View,
    Contract, ContractRuntime, Resources, SendMessageRequest,
};

use super::{state::Application, ApplicationContract, ApprovedAirDrop};

/// Tests if a valid airdrop claim is accepted and results in a message to execute the payment.
#[test]
fn accepts_new_claim() {
    let mut contract = create_and_instantiate_contract();
    let airdrop_id = AirDropId::from(Address::random());
    let destination_account = create_dummy_destination(0);

    let signature = "0x0000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000"
        .parse()
        .expect("Dummy signature is invalid");

    let claim = AirDropClaim {
        id: airdrop_id,
        signature,
        destination: destination_account,
    };

    let () = contract.execute_operation(claim).blocking_wait();

    let application_creator_chain_id = contract.runtime.application_creator_chain_id();
    let scheduled_messages = contract.runtime.created_send_message_requests();

    let expected_message = SendMessageRequest {
        destination: Destination::Recipient(application_creator_chain_id),
        authenticated: true,
        is_tracked: false,
        grant: Resources::default(),
        message: ApprovedAirDrop {
            id: airdrop_id,
            amount: Amount::ONE,
            destination: destination_account,
        },
    };

    assert_eq!(*scheduled_messages, vec![expected_message]);
}

/// Tests if an accepted airdrop leads to a call to transfer the tokens to the claimer.
#[test]
fn pays_accepted_airdrop() {
    let mut contract = create_and_instantiate_contract();
    let airdrop_id = AirDropId::from(Address::random());
    let amount = Amount::from_tokens(11);
    let destination = create_dummy_destination(0);

    let airdrop = ApprovedAirDrop {
        id: airdrop_id,
        amount,
        destination,
    };

    let application_id = contract.runtime.application_id();

    contract.runtime.set_call_application_handler(
        move |is_authenticated, target_application, operation| {
            assert!(is_authenticated);
            assert_eq!(target_application, create_dummy_token_id());
            assert_eq!(
                operation,
                bcs::to_bytes(&fungible::Operation::Transfer {
                    owner: AccountOwner::Application(application_id.forget_abi()),
                    amount,
                    target_account: destination,
                })
                .expect("`ApprovedAirDrop` message should be serializable")
            );

            bcs::to_bytes(&FungibleResponse::Ok).expect("Unit type should be serializable")
        },
    );

    let () = contract.execute_message(airdrop).blocking_wait();
}

/// Tests if the same airdrop pays the claimer once.
#[test]
#[should_panic(expected = "Airdrop has already been paid")]
fn rejects_repeated_airdrop() {
    let mut contract = create_and_instantiate_contract();
    let airdrop_id = AirDropId::from(Address::random());
    let amount = Amount::from_tokens(11);
    let first_destination = create_dummy_destination(0);
    let second_destination = create_dummy_destination(1);

    let first_claim = ApprovedAirDrop {
        id: airdrop_id,
        amount,
        destination: first_destination,
    };

    let second_claim = ApprovedAirDrop {
        id: airdrop_id,
        amount: Amount::ONE,
        destination: second_destination,
    };

    let application_id = contract.runtime.application_id();

    contract.runtime.set_call_application_handler(
        move |is_authenticated, target_application, operation| {
            assert!(is_authenticated);
            assert_eq!(target_application, create_dummy_token_id());
            assert_eq!(
                operation,
                bcs::to_bytes(&fungible::Operation::Transfer {
                    owner: AccountOwner::Application(application_id.forget_abi()),
                    amount,
                    target_account: first_destination,
                })
                .expect("`ApprovedAirDrop` message should be serializable")
            );

            bcs::to_bytes(&FungibleResponse::Ok).expect("Unit type should be serializable")
        },
    );

    let () = contract.execute_message(first_claim).blocking_wait();
    let () = contract.execute_message(second_claim).blocking_wait();
}

/// Creates an [`ApplicationContract`] instance and calls `instantiate` on it.
fn create_and_instantiate_contract() -> ApplicationContract {
    let runtime = ContractRuntime::new()
        .with_application_parameters(Parameters {
            token_id: create_dummy_token_id(),
        })
        .with_application_id(create_dummy_application_id("zk-airdrop", 1))
        .with_application_creator_chain_id(ChainId(CryptoHash::test_hash("creator chain")));

    let mut contract = ApplicationContract {
        state: Application::load(runtime.root_view_storage_context())
            .blocking_wait()
            .expect("Failed to read from mock key value store"),
        runtime,
    };

    contract.instantiate(()).blocking_wait();

    contract
}

/// Creates a dummy [`ApplicationId`] to use as the Fungible Token for testing.
fn create_dummy_token_id<Abi>() -> ApplicationId<Abi> {
    create_dummy_application_id("fungible token", 0)
}

/// Creates a dummy [`Account`] to use as a test destination for the airdropped tokens.
fn create_dummy_destination(index: usize) -> Account {
    Account {
        chain_id: ChainId(CryptoHash::test_hash(format!("destination chain {index}"))),
        owner: AccountOwner::User(Owner(CryptoHash::test_hash(format!(
            "destination owner {index}"
        )))),
    }
}
