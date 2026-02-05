# Solana Core-9 Security Bugs (Anchor examples)

This repository is a compact, educational reference of **nine real-world Solana security patterns**. Each module shows the same business intent implemented two ways:

- `vulnerable.rs` — intentionally broken logic
- `secure.rs` — corrected, defensive logic

The goal is to make security mistakes obvious, reproducible, and easy to explain in code reviews or audits.

## Structure

```
README.md
LICENSE
docs/deep-dive.md
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

## Contents

1. `examples/01_missing_authority` — Missing authority check
2. `examples/02_missing_signer` — Missing signer requirement
3. `examples/03_fake_account` — Fake account injection (owner not verified)
4. `examples/04_pda_misuse` — PDA misuse (seeds/bump not enforced)
5. `examples/05_state_order` — State mutation before validation
6. `examples/06_cpi_trust` — Unsafe CPI trust
7. `examples/07_cpi_reentrancy` — Re-entrancy via CPI
8. `examples/08_overflow` — Integer overflow/underflow
9. `examples/09_insecure_close` — Insecure account close / lamport drain

## How to read

Each folder includes:

- `README.md` — description of the vulnerability and fix
- `vulnerable.rs` — intentionally unsafe version with comments
- `secure.rs` — corrected version with comments

These examples are intentionally minimal and focus on **the security pattern**, not full program scaffolding.

## Deep dive

For a longer-form explanation of the patterns and why they matter, see [`docs/deep-dive.md`](docs/deep-dive.md).
