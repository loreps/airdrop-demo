import React from 'react';
import { gql, useMutation } from '@apollo/client';
import web3 from 'web3';
import { AirDropClaimMutation } from './qql/graphql';
import logo from './logo.svg';
import './App.css';

const CLAIM_AIRDROP = gql`
    mutation AirDropClaim($id: AirDropId!, $destination: FungibleAccount!, $signature: String!) {
        airDropClaim(id: $id, destination: $destination, signature: $signature)
    }
`;

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

  const handleSubmit = (event: { preventDefault: () => void }) => {
    event.preventDefault();
    claim({
      variables: {
        id: { externalAddress },
        signature: "0x0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        destination: {
          chainId: chainId,
          owner: `User:${owner}`,
        },
      },
    }).then((result) => console.log("Claimed " + result));
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
