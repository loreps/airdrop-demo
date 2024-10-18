import { CodegenConfig } from '@graphql-codegen/cli';

const host = "127.0.0.1";
const port = 8080;
const chainId = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
const appId = "7a9e87b047e171542f68d6cb13f22ff6e32acd8455c137ecccb1661333586ab5202cdef16ddc634d7e4daf9c9dbb81442d90b2f7b859a31d2b01a5b76987c411e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65070000000000000000000000";

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
