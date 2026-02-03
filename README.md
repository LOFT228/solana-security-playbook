# Solana Security Playbook

An educational reference for Solana developers that contrasts vulnerable instructions with secure alternatives. Each example is a small, focused program module with inline comments explaining the issue and the fix.

## Structure

- `programs/missing-account-validation`: Missing account owner/constraint checks.
- `programs/authority-check`: Incorrect authority verification.
- `programs/unsafe-arithmetic`: Unsafe arithmetic on balances/counters.
- `programs/unchecked-cpi`: CPI program ID validation and re-entrancy awareness.
- `programs/pda-seed-validation`: PDA seed/bump validation to prevent spoofing.

## How to Use

Read each `lib.rs` file. Each example contains:

- A **vulnerable** instruction.
- A **secure** instruction with comments explaining the fix.

## Deep Dive

A written deep-dive explaining the security patterns lives in `docs/deep-dive.md`.

## License

This repository is open source under the MIT License. See `LICENSE`.
