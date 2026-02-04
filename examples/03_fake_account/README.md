# 03 — Fake Account Injection (Owner Not Verified)

**Bug:** Program reads a state account but does not verify `account.owner == program_id`.

**Impact:** Attacker supplies a fake account with attacker-controlled data.

**Fix:** Enforce account ownership via Anchor `Account` types or explicit owner checks.
