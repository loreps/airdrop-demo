import React from 'react';
import { gql, useMutation } from '@apollo/client';
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
};

function App({ chainId, owner }: AppProps) {
  const [claim] = useMutation<AirDropClaimMutation>(CLAIM_AIRDROP, {
    onError: (error) => console.log(error),
    onCompleted: () => {},
  });

  const handleSubmit = (event: { preventDefault: () => void }) => {
    event.preventDefault();
    claim({
      variables: {
        id: { externalAddress: [] },
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
          <button type="submit">Claim</button>
        </form>
      </header>
    </div>
  );
}

export default App;
