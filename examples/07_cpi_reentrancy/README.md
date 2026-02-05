# 07 — Re‑entrancy via CPI

**Bug:** External CPI is invoked before state is locked/updated.  
**Impact:** Malicious program can re‑enter and drain funds or repeat operations.

**Fix:** Use a re‑entrancy guard and update state before CPI (or lock with a nonce), then validate after.
