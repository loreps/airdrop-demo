import React from 'react';
import { useState } from 'react';
import { useSyncProviders } from './hooks/useSyncProviders';

type AccountProviderProps = {
    children: any,
};

function AccountProvider({ children }: AccountProviderProps) {
    const [selectedProvider, setSelectedProvider] = useState<EIP6963ProviderDetail>();
    const [userAccount, setUserAccount] = useState<string | null>(null);
    const providers = useSyncProviders();

    if (selectedProvider == null) {
        if (providers.length > 0) {
            const provider = providers[0];

            setSelectedProvider(provider)

            provider.provider
                .request({ method: 'eth_requestAccounts' })
                .then((accountsUntyped: unknown) => {
                    let accounts = accountsUntyped as [string];

                    if (accounts.length > 0) {
                        setUserAccount(accounts[0]);
                    }
                })
                .catch((error: any) => {
                    console.log("Failed to connect to wallet: " + error);
                });
        }
    }

    if (userAccount !== null && selectedProvider !== null) {
        let elements = React.Children.map(children, (child) => {
            return React.cloneElement(child, { userAccount, web3Provider: selectedProvider });
        });

        return (<>{elements}</>);
    } else {
        return (<>{children}</>);
    }
}

export default AccountProvider;
