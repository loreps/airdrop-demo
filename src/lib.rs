// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use alloy_primitives::Address;
use indexmap::IndexMap;
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct AirDropClaim {
    pub id: AirDropId,
    pub destination: Account,
}

#[async_graphql::Scalar]
impl async_graphql::ScalarType for AirDropClaim {
    fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
        let async_graphql::Value::Object(mut fields) = value else {
            return Err(async_graphql::InputValueError::expected_type(value));
        };

        if fields.len() != 2 {
            return Err(async_graphql::InputValueError::custom(
                "`AirDropClaim` object must have exactly two fields: `id` and `destination`",
            ));
        }

        let Some(id_value) = fields.swap_remove("id") else {
            return Err(async_graphql::InputValueError::custom(
                "`AirDropClaim` object is missing an `id` field",
            ));
        };

        let id = match <AirDropId as async_graphql::ScalarType>::parse(id_value) {
            Ok(id) => id,
            Err(error) => return Err(error.propagate()),
        };

        let Some(destination_value) = fields.swap_remove("destination") else {
            return Err(async_graphql::InputValueError::custom(
                "`AirDropClaim` object is missing an `destination` field",
            ));
        };

        let destination =
            match <Account as async_graphql::InputType>::parse(Some(destination_value)) {
                Ok(destination) => destination,
                Err(error) => return Err(error.propagate()),
            };

        Ok(AirDropClaim { id, destination })
    }

    fn to_value(&self) -> async_graphql::Value {
        let mut fields = IndexMap::new();

        let id = async_graphql::ScalarType::to_value(&self.id);
        let destination = async_graphql::InputType::to_value(&self.destination);

        fields.insert(async_graphql::Name::new("id"), id);
        fields.insert(async_graphql::Name::new("destination"), destination);

        async_graphql::Value::Object(fields)
    }
}

/// The [EIP-155] constant for the Ethereum mainnet.
///
/// [EIP-155]: https://eips.ethereum.org/EIPS/eip-155
pub const ETHEREUM_MAINNET_CHAIN_ID: alloy_primitives::aliases::ChainId = 1;
