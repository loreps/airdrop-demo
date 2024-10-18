import { CodegenConfig } from '@graphql-codegen/cli';

const host = "127.0.0.1";
const port = 8080;
const chainId = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
const appId = "4e1c418b64d00c455004a03d68fb7bf6e9a9748625a960690c6476496e793f88f640246b3dfb9545413894f30565f80a4d1e78de6d91696f3d2fe2d4a042539fe476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65040000000000000000000000";

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
