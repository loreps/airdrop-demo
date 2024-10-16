import React from 'react';
import { gql, useMutation } from '@apollo/client';
import web3, { Web3 } from 'web3';
import { AirDropClaimMutation } from './qql/graphql';
import logo from './logo.svg';
import './App.css';

const CLAIM_AIRDROP = gql`
    mutation AirDropClaim($destination: FungibleAccount!, $signature: String!, $apiToken: String!) {
        airDropClaim(destination: $destination, signature: $signature, apiToken: $apiToken)
    }
`;

const ETHEREUM_MAINNET_CHAIN_ID = 1;

type AppProps = {
  appId: string,
  chainId: string,
  owner: string,
  userAccount?: string,
  web3Provider?: EIP6963ProviderDetail,
};

function App({ appId, chainId, owner, userAccount, web3Provider }: AppProps) {
  const [claim] = useMutation<AirDropClaimMutation>(CLAIM_AIRDROP, {
    onError: (error) => console.log(error),
    onCompleted: () => {},
  });

  const externalAddress: Array<number> = Array.from(web3.utils.hexToBytes(userAccount || ''));

  const claimer = {
    chainId,
    owner: `User:${owner}`,
  };

  const handleSubmit = (event: { preventDefault: () => void }) => {
    event.preventDefault();

    if (userAccount == null) {
      throw Error('Missing user account. The Claim button should have been disabled');
    }
    if (web3Provider == null) {
      throw Error('Missing Web3 provider. The Claim button should have been disabled');
    }

    const web3 = new Web3(web3Provider.provider);

    web3.eth.signTypedData(userAccount, {
      domain: {
        name: "Linera AirDrop demo",
        version: "0.0.1",
        chainId: ETHEREUM_MAINNET_CHAIN_ID,
      },
      primaryType: "AirDropClaim",
      types: {
        EIP712Domain: [
          { name: "name", type: "string" },
          { name: "version", type: "string" },
          { name: "chainId", type: "uint256" },
        ],
        AirDropClaim: [
          { name: "appId", type: "string" },
          { name: "claimer", type: "FungibleAccount" },
        ],
        FungibleAccount: [
          { name: "chainId", type: "string" },
          { name: "owner", type: "string" },
        ],
      },
      message: {
        appId,
        claimer,
      },
    }).then((signature) => {
        claim({
          variables: {
            signature,
            destination: claimer,
          },
        }).then((result) => console.log("Claimed " + result));
    }).catch((error: any) => {
        console.log("Failed to obtain signature: " + error);
    });
  };

  return (
    <div className="App">
      <header className="App-header">
        <form onSubmit={handleSubmit}>
          <button type="submit" disabled={userAccount == null || web3Provider == null}>
            Claim
          </button>
        </form>
      </header>
    </div>
  );
}

export default App;
