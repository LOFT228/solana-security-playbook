# Deep Dive: Core 9 Solana Security Patterns

This document explains *why each pattern is dangerous* and how the secure variant fixes it. The goal is clarity, not size.

## 1) Missing Authority Check
**Bug:** A config/vault update does not verify the stored authority.  
**Result:** Any caller can change critical state.  
**Fix:** Require a signer and compare against `config.authority` (or use `has_one`).

## 2) Missing Signer Requirement
**Bug:** Authority key is compared but the account is not a signer.  
**Result:** Attacker can pass the authority pubkey without a signature.  
**Fix:** Make the authority a `Signer<'info>`.

## 3) Fake Account Injection (Owner Not Verified)
**Bug:** Program reads state from an unverified account.  
**Result:** Attacker supplies arbitrary data to bypass checks.  
**Fix:** Use Anchor `Account<T>` types or explicitly check `account.owner`.

## 4) PDA Misuse (Seeds / Bump Not Enforced)
**Bug:** PDA is accepted without enforcing seeds/bump.  
**Result:** Attacker swaps in a fake account.  
**Fix:** Use `seeds` + `bump` constraints or derive and verify PDA manually.

## 5) State Mutation Before Validation
**Bug:** Program mutates state before doing all checks.  
**Result:** State can be corrupted if checks fail or re-entrancy happens.  
**Fix:** Validate first, then mutate (“checks-before-effects”).

## 6) Unsafe CPI Trust
**Bug:** Any CPI target is accepted.  
**Result:** Malicious programs run during your instruction.  
**Fix:** Allowlist CPI program IDs or store a trusted program in config.

## 7) CPI Re-entrancy
**Bug:** CPI happens while state is mutable and unguarded.  
**Result:** Re-entrancy allows repeated withdrawals/mints.  
**Fix:** Use a lock flag and set it before CPI; clear it after.

## 8) Integer Overflow / Underflow
**Bug:** Unchecked math on balances or counters.  
**Result:** Wraparound enables bypasses or resets.  
**Fix:** Use checked arithmetic and return explicit errors.

## 9) Insecure Account Close / Lamport Drain
**Bug:** Closing accounts without verifying authority.  
**Result:** Attackers drain lamports or grief.  
**Fix:** Require the authority to sign and match stored data.
