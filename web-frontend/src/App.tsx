import React from 'react';
import { gql, useMutation } from '@apollo/client';
import web3 from 'web3';
import { AirDropClaimMutation } from './qql/graphql';
import logo from './logo.svg';
import './App.css';

const CLAIM_AIRDROP = gql`
    mutation AirDropClaim($id: AirDropId!, $destination: FungibleAccount!) {
        airDropClaim(id: $id, destination: $destination)
    }
`;

type AppProps = {
  chainId: string,
  owner: string,
  userAccount?: string,
};

function App({ chainId, owner, userAccount }: AppProps) {
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
          <button type="submit" disabled={userAccount === null}>Claim</button>
        </form>
      </header>
    </div>
  );
}

export default App;
