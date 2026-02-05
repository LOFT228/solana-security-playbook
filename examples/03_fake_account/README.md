# 03 — Fake Account Injection (Owner Not Verified)

**Bug:** The program reads a state account without verifying the account owner.  
**Impact:** Attackers can pass a fake account with attacker‑controlled data.

**Fix:** Enforce `account.owner == program_id` or use Anchor `Account<...>` to validate ownership.
