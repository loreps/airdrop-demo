// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use alloy_primitives::Address;
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
    type Query = async_graphql::Request;
    type QueryResponse = async_graphql::Response;
}

/// The shared parameters that are specified when the application is instantiated.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub token_id: ApplicationId<FungibleTokenAbi>,
}

/// The information necessary to identify an airdrop.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AirDropId {
    external_address: Address,
}

impl From<Address> for AirDropId {
    fn from(external_address: Address) -> Self {
        AirDropId { external_address }
    }
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for AirDropId {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        match value {
            async_graphql::Value::List(bytes_list) => {
                let parse_bytes_result = bytes_list
                    .into_iter()
                    .map(<u8 as async_graphql::ScalarType>::parse)
                    .collect::<Result<Vec<u8>, _>>();

                let prepare_bytes_result = parse_bytes_result.and_then(|bytes| {
                    <[u8; 20]>::try_from(bytes).map_err(|_| {
                        async_graphql::InputValueError::custom(
                            "Ethereum address needs exactly 20 bytes",
                        )
                    })
                });

                let external_address = match prepare_bytes_result {
                    Ok(bytes) => Address::from(bytes),
                    Err(error) => return Err(error.propagate()),
                };

                Ok(AirDropId { external_address })
            }
            async_graphql::Value::String(address_string) => {
                let hex_string = address_string.strip_prefix("0x").unwrap_or(&address_string);
                let external_address = Address::from_str(hex_string)?;

                Ok(AirDropId { external_address })
            }
            _ => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> async_graphql::Value {
        self.external_address
            .to_checksum(Some(ETHEREUM_MAINNET_CHAIN_ID))
            .into()
    }
}

/// An airdrop claim.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, async_graphql::SimpleObject)]
pub struct AirDropClaim {
    pub id: AirDropId,
    pub destination: Account,
}

/// The [EIP-155] constant for the Ethereum mainnet.
///
/// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
pub const ETHEREUM_MAINNET_CHAIN_ID: alloy_primitives::aliases::ChainId = 1;
