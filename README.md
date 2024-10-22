# Linera & Space-and-Time Airdrop demo

This is an example [Linera](https://linera.io) application that shows how to use
[Space-and-Time](https://spaceandtime.io) in order to determine if an Ethereum address is eligible
to an airdrop of some arbitrary tokens.

## Application Design

The microchain which instantiates the application becomes responsible for distributing tokens to the
airdrop claimers. Any microchain can be used to claim an airdrop. When the `AirDropClaim` operation
is added to a block, the application will check the claimer's eligibility, and if accepted will
send an `ApprovedAirDrop` message to the creator chain. The creator chain is responsible for
managing the tokens, and ensuring each claim is only paid once.

This design allows the eligibility verification of an unlimited of claims to run in parallel, while
the creator chain focuses on distributing tokens and preventing replay attacks.

## Eligibility Verification

For each claim, Space-and-Time's network is queried using the
[Gateway](https://docs.spaceandtime.io/docs/secrets-proxy) to check that the claim is eligible to
the airdrop. The query simply checks if the address had a minimum balance at a specific snapshot
block height.

In order to execute a claim, an API access token must be provided. This token is used by the client
proposing the block as well as each validator that validates the block.

The application performs the query from the contract using the service as an oracle. This is needed
because the service will handle the response and return only the relevant parts, which is what is
tracked and agreeded upon between the validators. Any sources of non-determinism (e.g., the HTTP
"Date" header in the response) is filtered out.

## Web Interface

A minimal web-interface to the application is provided. It communicates with an Ethereum wallet
(currently it has only been tested with MetaMask) to sign a message using the claimer's address.
This ensures that only the owner of that address can claim the airdrop tokens for that address.

## Future Work

### Verifying Proofs

Usage of the Gateway should be replaced with verification of the zero-knowledge proofs inside the
application. This would remove the need of the API access token, and allow claimers to obtain query
proofs through their preferred method.

### Sharding the Token Distribution

The responsibilities of the creator chain could be sharded into many microchains, where each one
handles a range of claimer addresses.
