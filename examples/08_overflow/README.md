# 08 — Integer Overflow / Underflow

**Bug:** Uses unchecked arithmetic on balances or counters.

**Impact:** Overflow resets values or allows bypassing limits.

**Fix:** Use `checked_add`, `checked_sub`, and return errors on overflow.
