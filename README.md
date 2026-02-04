# Solana Security Playbook — Core 9 Patterns

A curated, practical reference of the **nine most common Solana security bugs**, each shown as a **vulnerable vs secure** example with clear comments. The goal is to make the security pitfalls obvious and easy to learn.

## Why this structure works

- **Small, focused examples** — each pattern is one instruction pair.
- **Clear contrast** — vulnerable and secure implementations share the same intent.
- **Readable** — each folder includes a short README with the “why.”

## Core 9 Patterns

1. **Missing Authority Check** — anyone can change config/state.
2. **Missing Signer Requirement** — authority key checked, but not a signer.
3. **Fake Account Injection** — account owner not verified.
4. **PDA Misuse** — seeds/bump not enforced.
5. **State Mutation Before Validation** — checks happen too late.
6. **Unsafe CPI Trust** — calling arbitrary programs without validation.
7. **CPI Re-entrancy** — no re-entrancy guard around mutable state.
8. **Integer Overflow/Underflow** — unchecked math on balances.
9. **Insecure Account Close** — lamport drain due to missing authority checks.

## Repository Layout

```
examples/
  01_missing_authority/
  02_missing_signer/
  03_fake_account/
  04_pda_misuse/
  05_state_order/
  06_cpi_trust/
  07_cpi_reentrancy/
  08_overflow/
  09_insecure_close/
```

Each folder contains:
- `vulnerable.rs`
- `secure.rs`
- `README.md` (one-page explanation)

## Deep Dive

See `docs/deep-dive.md` for the written explanation of each pattern.

## License

MIT — see `LICENSE`.
