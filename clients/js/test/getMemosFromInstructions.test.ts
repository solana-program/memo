import { getUtf8Encoder, type Address, type Instruction } from '@solana/kit';
import { describe, expect, it } from 'vitest';

import {
    getMemosFromInstructions,
    LEGACY_MEMO_PROGRAM_ADDRESS_V1,
    LEGACY_MEMO_PROGRAM_ADDRESS_V3,
    MEMO_PROGRAM_ADDRESS,
} from '../src';

const NON_MEMO_PROGRAM_ADDRESS = '11111111111111111111111111111111' as Address<'11111111111111111111111111111111'>;

function memoInstruction(programAddress: Address, memo: string): Instruction {
    return { programAddress, data: getUtf8Encoder().encode(memo) };
}

describe('getMemosFromInstructions', () => {
    it('extracts memos emitted by every supported program version', () => {
        const instructions = [
            memoInstruction(LEGACY_MEMO_PROGRAM_ADDRESS_V1, 'from v1'),
            memoInstruction(LEGACY_MEMO_PROGRAM_ADDRESS_V3, 'from v3'),
            memoInstruction(MEMO_PROGRAM_ADDRESS, 'from v4'),
        ];

        expect(getMemosFromInstructions(instructions)).toEqual([
            {
                memo: 'from v1',
                bytes: getUtf8Encoder().encode('from v1'),
                programAddress: LEGACY_MEMO_PROGRAM_ADDRESS_V1,
                index: 0,
            },
            {
                memo: 'from v3',
                bytes: getUtf8Encoder().encode('from v3'),
                programAddress: LEGACY_MEMO_PROGRAM_ADDRESS_V3,
                index: 1,
            },
            {
                memo: 'from v4',
                bytes: getUtf8Encoder().encode('from v4'),
                programAddress: MEMO_PROGRAM_ADDRESS,
                index: 2,
            },
        ]);
    });

    it('ignores non-memo instructions and preserves the original index', () => {
        const instructions = [
            { programAddress: NON_MEMO_PROGRAM_ADDRESS } satisfies Instruction,
            memoInstruction(MEMO_PROGRAM_ADDRESS, 'Hello world!'),
            { programAddress: NON_MEMO_PROGRAM_ADDRESS } satisfies Instruction,
        ];

        expect(getMemosFromInstructions(instructions)).toEqual([
            {
                memo: 'Hello world!',
                bytes: getUtf8Encoder().encode('Hello world!'),
                programAddress: MEMO_PROGRAM_ADDRESS,
                index: 1,
            },
        ]);
    });

    it('returns multiple memos in order', () => {
        const instructions = [
            memoInstruction(MEMO_PROGRAM_ADDRESS, 'first'),
            { programAddress: NON_MEMO_PROGRAM_ADDRESS } satisfies Instruction,
            memoInstruction(MEMO_PROGRAM_ADDRESS, 'second'),
        ];

        expect(getMemosFromInstructions(instructions).map(m => m.memo)).toEqual(['first', 'second']);
    });

    it('decodes multi-byte UTF-8 memos', () => {
        const instructions = [memoInstruction(MEMO_PROGRAM_ADDRESS, 'gm 🌞 café')];

        expect(getMemosFromInstructions(instructions)[0]?.memo).toBe('gm 🌞 café');
    });

    it('yields an empty string and empty bytes for a memo instruction with no data', () => {
        const instructions = [{ programAddress: MEMO_PROGRAM_ADDRESS } satisfies Instruction];

        expect(getMemosFromInstructions(instructions)).toEqual([
            { memo: '', bytes: new Uint8Array(), programAddress: MEMO_PROGRAM_ADDRESS, index: 0 },
        ]);
    });

    it('returns an empty array when there are no memo instructions', () => {
        const instructions = [{ programAddress: NON_MEMO_PROGRAM_ADDRESS } satisfies Instruction];

        expect(getMemosFromInstructions(instructions)).toEqual([]);
    });
});
