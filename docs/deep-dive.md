# Deep Dive: Core-9 Solana Security Bugs

This document expands on the nine patterns in this repository. Each section explains **why the bug happens**, **how it is exploited**, and **what a correct fix looks like**.

## 1) Missing Authority Check
**Why it happens:** Developers store an `authority` in state but forget to compare it to the caller.  
**Exploit:** Anyone can mutate critical state (vault config, admin settings).  
**Fix:** Require the caller’s key to match state authority (e.g., `has_one = authority` and/or explicit key equality).

## 2) Missing Signer Requirement
**Why it happens:** The correct account is passed, but the program never enforces `is_signer`.  
**Exploit:** An attacker can invoke the instruction without the victim’s private key.  
**Fix:** Mark the account as `Signer` (Anchor) or manually check `is_signer`.

## 3) Fake Account Injection (Owner Not Verified)
**Why it happens:** Code assumes an account is “the right one” because it has the correct data layout.  
**Exploit:** Attacker passes a look‑alike account owned by a different program with malicious values.  
**Fix:** Enforce `account.owner == program_id` and/or Anchor `Account<...>` constraints.

## 4) PDA Misuse (Seeds / Bump Not Enforced)
**Why it happens:** PDA derivation is assumed but not verified on-chain.  
**Exploit:** Attacker supplies a non‑PDA account they control.  
**Fix:** Use `seeds`/`bump` constraints or `Pubkey::find_program_address` checks.

## 5) State Mutation Before Validation
**Why it happens:** Code updates state before verifying all conditions.  
**Exploit:** Partial state changes occur even if later checks fail; attackers can use this to bypass invariants.  
**Fix:** Validate all inputs and invariants first, mutate last.

## 6) Unsafe CPI Trust
**Why it happens:** Programs treat CPI like a local function call and trust outputs.  
**Exploit:** A malicious program returns unexpected values or performs side effects.  
**Fix:** Validate CPI program IDs, verify post‑conditions, and minimize trust in external state.

## 7) Re‑entrancy via CPI
**Why it happens:** A program invokes an external program that can re‑enter and call back before state is locked.  
**Exploit:** Repeated withdrawals, minting, or state inconsistencies.  
**Fix:** Use re‑entrancy guards, update state before CPI (or lock with nonce), and validate invariants after.

## 8) Integer Overflow / Underflow
**Why it happens:** Using `+`, `-`, `*` on unchecked values.  
**Exploit:** Balance wraps, turning small amounts into huge balances or draining funds.  
**Fix:** Use checked arithmetic (`checked_add`, `checked_sub`).

## 9) Insecure Account Close / Lamport Drain
**Why it happens:** Close/withdraw logic lacks authority checks.  
**Exploit:** Attacker closes accounts and steals lamports.  
**Fix:** Require the owner/authority to sign and match stored authority.
