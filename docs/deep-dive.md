# Deep Dive: Solana Security Patterns

This document explains the security patterns demonstrated in the example programs. Each section references a vulnerable instruction and its secure counterpart, focusing on the reasoning behind the fix.

## 1) Missing Account Validation

**Problem:** If a program accepts generic `AccountInfo` accounts without validating ownership or mint, an attacker can substitute their own accounts. This leads to unauthorized token transfers or state writes.

**Fix:** Use Anchor `Account` types with explicit constraints (e.g., token owner, mint) or explicit runtime checks. Tie vault accounts to a PDA or config state and verify the mint and owner before performing a CPI transfer.

## 2) Incorrect Authority Checks

**Problem:** Comparing a public key without enforcing a signature lets attackers pass the authority's public key and bypass the check.

**Fix:** Require the authority as a `Signer<'info>` and verify it matches the stored authority key. This guarantees the authority actually signed the transaction.

## 3) Unsafe Arithmetic

**Problem:** Direct arithmetic on `u64` can overflow and wrap, enabling attackers to reset counters or bypass limits.

**Fix:** Use `checked_add`, `checked_sub`, and other safe math routines. Return a custom error on overflow to halt execution.

## 4) Unchecked CPI + Re-entrancy Risks

**Problem:** Calling arbitrary CPI programs is dangerous. A malicious program can re-enter and mutate state multiple times, or the program could be tricked into calling an unintended program.

**Fix:** Validate the CPI program ID against an allowlist or stored config. Add a simple re-entrancy lock around state changes to prevent nested execution.

## 5) PDA Seed / Bump Validation

**Problem:** If you don't validate PDA seeds (or rely on user-supplied bumps), attackers can pass fake accounts that look legitimate.

**Fix:** Use Anchor `seeds` and `bump` constraints to force PDA derivation on-chain. This guarantees the account is the intended PDA.

## Recommended Usage

- Keep examples small and readable.
- Add tests that attempt the exploit and show the fix.
- Expand each pattern to cover both Anchor and Pinocchio where possible.
