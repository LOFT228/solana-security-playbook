# 07 — CPI Re-entrancy

**Bug:** No re-entrancy guard around mutable state when calling CPI.

**Impact:** A malicious program can re-enter and perform multiple withdrawals.

**Fix:** Use a lock flag or checks-before-effects pattern.
