# 09 — Insecure Account Close / Lamport Drain

**Bug:** Anyone can close a state account and drain its lamports.

**Impact:** Attackers can steal rent or grief by closing accounts.

**Fix:** Require authority checks before closing accounts.
