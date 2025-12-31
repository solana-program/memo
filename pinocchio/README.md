# `p-memo`

A `pinocchio`-based Memo program.

## Overview

`p-memo` is a reimplementation of the SPL Memo program using [`pinocchio`](https://github.com/anza-xyz/pinocchio). The program uses at most `~5%` of the compute units used by the current Memo program when signers are present; even when there are no signers, it needs only `~14%` of the current Memo program compute units. This efficiency is achieved by a combination of:

1. `pinocchio` "lazy" entrypoint
2. `sol_log_pubkey` syscall to log pubkey values
3. Direct `sol_log_` syscalls for efficient logging

Since it uses the syscall to log pubkeys, the output of the program is slightly different while loging the same information:

```
1: Program PMemo11111111111111111111111111111111111111 invoke [1]
2: Program log: Signed by:
3: Program log: 1111111QLbz7JHiBTspS962RLKV8GndWFwiEaqKM
4: Program log: why does spl memo use 36000 cus to print len 60 msg of ascii
5: Program PMemo11111111111111111111111111111111111111 consumed 537 of 1400000 compute units
6: Program PMemo11111111111111111111111111111111111111 success
```

Logging begins with entry into the program (`line 1`). Then there is a separate log to start the signers section (`line 2`); this is only present if there are signer accounts. After that there will be one line for each signer account (`line 3`), followed by the memo UTF-8 text (`line 4`). The program ends with the status of the instruction (`lines 5-6`).

## Performance

CU comsumption:

| \# signers | p-memo      | SPL Memo |
| ---------- | ----------- | -------- |
| 0          | 283 (`14%`) | 2,022    |
| 1          | 537 (`4%`)  | 13,525   |
| 2          | 654 (`3%`)  | 25,111   |
| 3          | 771 (`2%`)  | 36,406   |

> [!NOTE]
> Using Solana CLI `v2.2.15`.

## Building

To build the program from its directory:

```bash
cargo build-sbf
```

## Testing

To run the tests (after building the program):

```bash
SBF_OUT_DIR=../target/deploy cargo test
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
