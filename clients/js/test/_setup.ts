import path from 'node:path';

import { createClient, lamports } from '@solana/kit';
import { litesvm } from '@solana/kit-plugin-litesvm';
import { airdropSigner, generatedSigner } from '@solana/kit-plugin-signer';

import { MEMO_PROGRAM_ADDRESS, memoProgram } from '../src';

const MEMO_BINARY_PATH = path.resolve(__dirname, '..', '..', '..', 'target', 'deploy', 'spl_memo.so');

export const createTestClient = () => {
    return createClient()
        .use(generatedSigner())
        .use(litesvm())
        .use(airdropSigner(lamports(1_000_000_000n)))
        .use(client => {
            // Load the memo program into the LiteSVM instance from its compiled
            // `.so` file. Must run after the `litesvm()` plugin so that
            // `client.svm` is available.
            client.svm.addProgramFromFile(MEMO_PROGRAM_ADDRESS, MEMO_BINARY_PATH);
            return client;
        })
        .use(memoProgram());
};
