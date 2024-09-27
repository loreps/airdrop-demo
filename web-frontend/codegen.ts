import { CodegenConfig } from '@graphql-codegen/cli';

const host = "localhost";
const port = 8080;
const chainId = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65";
const appId = "44ba9f5c0eb3322b7c057edeaadb229d18c3dba16dde89f92fa8697ab02d6d5b5909237fea350cd68f799aef848c49cef571c38a4a394fc989800b7f38b52a14e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a651d0000000000000000000000";

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
