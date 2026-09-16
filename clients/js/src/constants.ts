import { type Address } from '@solana/kit';

import { MEMO_PROGRAM_ADDRESS } from './generated';

/**
 * The address of the version 1 Memo program.
 *
 * This is a legacy address, kept for reading memos emitted by older transactions.
 * Use {@link MEMO_PROGRAM_ADDRESS} when building new memo instructions.
 */
export const LEGACY_MEMO_PROGRAM_ADDRESS_V1 =
    'Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo' as Address<'Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo'>;

/**
 * The address of the version 3 Memo program.
 *
 * This is a legacy address, kept for reading memos emitted by older transactions.
 * Use {@link MEMO_PROGRAM_ADDRESS} when building new memo instructions.
 */
export const LEGACY_MEMO_PROGRAM_ADDRESS_V3 =
    'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr' as Address<'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'>;

/**
 * Every Memo program address ever deployed, ordered from oldest (v1) to newest (v4).
 *
 * Consumers that need to detect memos should match against every address in this
 * list rather than a single program address, since a transaction may carry memos
 * emitted under any historical version of the program. The newest address (v4) is
 * exported separately as `MEMO_PROGRAM_ADDRESS` and should be used to build new
 * memo instructions.
 *
 * @example
 * ```ts
 * import { SUPPORTED_MEMO_PROGRAM_ADDRESSES } from '@solana-program/memo';
 *
 * const isMemoInstruction = SUPPORTED_MEMO_PROGRAM_ADDRESSES.includes(instruction.programAddress);
 * ```
 */
export const SUPPORTED_MEMO_PROGRAM_ADDRESSES = Object.freeze([
    LEGACY_MEMO_PROGRAM_ADDRESS_V1,
    LEGACY_MEMO_PROGRAM_ADDRESS_V3,
    MEMO_PROGRAM_ADDRESS,
] as const) satisfies readonly Address[];
