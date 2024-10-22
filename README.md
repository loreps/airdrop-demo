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

## Future Work

### Sharding the Token Distribution

The responsibilities of the creator chain could be sharded into many microchains, where each one
handles a range of claimer addresses.
