# 09 — Insecure Account Close / Lamport Drain

**Bug:** Accounts can be closed without verifying the authority.  
**Impact:** Attackers can close accounts and steal lamports.

**Fix:** Require the authority to sign and match the stored authority before closing.
