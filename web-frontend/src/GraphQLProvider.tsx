import { ApolloClient, ApolloProvider, HttpLink, InMemoryCache, split } from '@apollo/client';
import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from 'graphql-ws';
import { getMainDefinition } from '@apollo/client/utilities';

type GraphQLProviderProps = {
 chainId: string, applicationId: string, host: string, port: string, children: any };

function GraphQLProvider({ chainId, applicationId, host, port, children }: GraphQLProviderProps) {
    let client = apolloClient(chainId, applicationId, host, port);
    return <ApolloProvider client={client}>{children}</ApolloProvider>;
}

function apolloClient(chainId: string, applicationId: string, host: string, port: string) {
    const wsLink = new GraphQLWsLink(
        createClient({
            url: `ws://${host}:${port}/ws`,
        })
    );

    const httpLink = new HttpLink({
        uri: `http://${host}:${port}/chains/${chainId}/applications/${applicationId}`,
    });

    const splitLink = split(
        ({ query }) => {
            const definition = getMainDefinition(query);
            return definition.kind === "OperationDefinition"
                && definition.operation === "subscription";
        },
        wsLink,
        httpLink,
    );

    return new ApolloClient({
        link: splitLink,
        cache: new InMemoryCache(),
    });
}

export default GraphQLProvider;
