import { getUtf8Decoder, type Address, type Instruction, type ReadonlyUint8Array } from '@solana/kit';

import { SUPPORTED_MEMO_PROGRAM_ADDRESSES } from './constants';

/**
 * A memo extracted from an instruction by {@link getMemosFromInstructions}.
 */
export type ExtractedMemo = {
    /** The UTF-8 decoded memo text. Instructions with no data yield an empty string. */
    memo: string;
    /** The raw, undecoded instruction data. Instructions with no data yield an empty array. */
    bytes: ReadonlyUint8Array;
    /** The address of the Memo program that emitted this memo. */
    programAddress: Address;
    /** The index of the source instruction within the array passed to the helper. */
    index: number;
};

/**
 * Extracts every memo from an array of instructions, regardless of which version of
 * the Memo program emitted it.
 *
 * The Memo program has been deployed under several addresses over time (see
 * {@link SUPPORTED_MEMO_PROGRAM_ADDRESSES}). This helper matches instructions against
 * all of them, so consumers do not need to know about the individual program
 * addresses. Matching instructions are decoded as UTF-8, matching the program's own
 * contract; the original ordering is preserved and each result carries the source
 * program address and instruction index.
 *
 * @example
 * ```ts
 * import { getMemosFromInstructions } from '@solana-program/memo';
 *
 * const memos = getMemosFromInstructions(transactionMessage.instructions);
 * // => [{ memo: 'Hello world!', programAddress: 'Memo4c2p…', index: 1 }]
 * ```
 */
export function getMemosFromInstructions(instructions: readonly Instruction[]): ExtractedMemo[] {
    const decoder = getUtf8Decoder();
    const memos: ExtractedMemo[] = [];

    const supportedAddresses: readonly Address[] = SUPPORTED_MEMO_PROGRAM_ADDRESSES;

    instructions.forEach((instruction, index) => {
        if (!supportedAddresses.includes(instruction.programAddress)) {
            return;
        }

        const bytes = instruction.data ?? new Uint8Array();
        memos.push({
            memo: decoder.decode(bytes),
            bytes,
            programAddress: instruction.programAddress,
            index,
        });
    });

    return memos;
}
