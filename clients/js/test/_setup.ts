import { createClient, lamports } from '@solana/kit';
import { solanaLocalRpc } from '@solana/kit-plugin-rpc';
import { airdropSigner, generatedSigner } from '@solana/kit-plugin-signer';

import { memoProgram } from '../src';

export const createTestClient = () => {
    return createClient()
        .use(generatedSigner())
        .use(solanaLocalRpc())
        .use(airdropSigner(lamports(1_000_000_000n)))
        .use(memoProgram());
};
