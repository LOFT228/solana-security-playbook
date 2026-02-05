# 08 — Integer Overflow / Underflow

**Bug:** Arithmetic is done with `+`/`-` without checks.  
**Impact:** Balances can wrap around and become huge or zero.

**Fix:** Use checked arithmetic (`checked_add`, `checked_sub`) and handle failures.
