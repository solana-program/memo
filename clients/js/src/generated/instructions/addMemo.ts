/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Address } from '@solana/addresses';
import {
  Codec,
  Decoder,
  Encoder,
  combineCodec,
  getStringDecoder,
  getStringEncoder,
  getStructDecoder,
  getStructEncoder,
} from '@solana/codecs';
import {
  IAccountMeta,
  IInstruction,
  IInstructionWithAccounts,
  IInstructionWithData,
} from '@solana/instructions';

export type AddMemoInstruction<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr',
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<TRemainingAccounts>;

export type AddMemoInstructionWithSigners<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr',
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<TRemainingAccounts>;

export type AddMemoInstructionData = { memo: string };

export type AddMemoInstructionDataArgs = AddMemoInstructionData;

export function getAddMemoInstructionDataEncoder(): Encoder<AddMemoInstructionDataArgs> {
  return getStructEncoder([['memo', getStringEncoder()]]);
}

export function getAddMemoInstructionDataDecoder(): Decoder<AddMemoInstructionData> {
  return getStructDecoder([['memo', getStringDecoder()]]);
}

export function getAddMemoInstructionDataCodec(): Codec<
  AddMemoInstructionDataArgs,
  AddMemoInstructionData
> {
  return combineCodec(
    getAddMemoInstructionDataEncoder(),
    getAddMemoInstructionDataDecoder()
  );
}

export type AddMemoInput = {
  memo: AddMemoInstructionDataArgs['memo'];
};

export type AddMemoInputWithSigners = {
  memo: AddMemoInstructionDataArgs['memo'];
};

export function getAddMemoInstruction<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'
>(input: AddMemoInputWithSigners): AddMemoInstructionWithSigners<TProgram>;
export function getAddMemoInstruction<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'
>(input: AddMemoInput): AddMemoInstruction<TProgram>;
export function getAddMemoInstruction<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'
>(input: AddMemoInput): IInstruction {
  // Program address.
  const programAddress =
    'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr' as Address<'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'>;

  // Original args.
  const args = { ...input };

  const instruction = getAddMemoInstructionRaw(
    args as AddMemoInstructionDataArgs,
    programAddress
  );

  return instruction;
}

export function getAddMemoInstructionRaw<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr',
  TRemainingAccounts extends Array<IAccountMeta<string>> = []
>(
  args: AddMemoInstructionDataArgs,
  programAddress: Address<TProgram> = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr' as Address<TProgram>,
  remainingAccounts?: TRemainingAccounts
) {
  return {
    accounts: remainingAccounts ?? [],
    data: getAddMemoInstructionDataEncoder().encode(args),
    programAddress,
  } as AddMemoInstruction<TProgram, TRemainingAccounts>;
}

export type ParsedAddMemoInstruction<
  TProgram extends string = 'MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr'
> = {
  programAddress: Address<TProgram>;
  data: AddMemoInstructionData;
};

export function parseAddMemoInstruction<TProgram extends string>(
  instruction: IInstruction<TProgram> & IInstructionWithData<Uint8Array>
): ParsedAddMemoInstruction<TProgram> {
  return {
    programAddress: instruction.programAddress,
    data: getAddMemoInstructionDataDecoder().decode(instruction.data),
  };
}
