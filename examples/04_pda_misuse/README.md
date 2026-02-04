# 04 — PDA Misuse (Seeds / Bump Not Enforced)

**Bug:** PDA account is accepted without verifying seeds/bump.

**Impact:** Attacker supplies a fake account to bypass authorization or steal funds.

**Fix:** Use `seeds` and `bump` constraints or explicit PDA derivation checks.
