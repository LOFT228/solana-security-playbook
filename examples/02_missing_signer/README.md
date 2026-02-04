# 02 — Missing Signer Requirement

**Bug:** Authority key is checked but the account is not required to sign.

**Impact:** Attacker passes the authority pubkey without a signature.

**Fix:** Use `Signer<'info>` (Anchor) or explicit signature checks.
