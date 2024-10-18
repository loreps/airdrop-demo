import { CodegenConfig } from '@graphql-codegen/cli';

const host = "127.0.0.1";
const port = 8080;
const chainId = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
const appId = "1a95e82634d8a3af13537c617915af9ed919263984315e5526599b593278d31a1f2ecb704f211a852e52b2188f2a98037c8309fd5f7e4f31c6ce5b9916eb1325e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65010000000000000000000000";

const config: CodegenConfig = {
    schema: `http://${host}:${port}/chains/${chainId}/applications/${appId}`,
    documents: ['src/**/*.{ts,tsx}'],
    generates: {
        './src/qql/': {
            preset: 'client',
            plugins: [],
        }
    },
    ignoreNoDocuments: true,
};

export default config;
