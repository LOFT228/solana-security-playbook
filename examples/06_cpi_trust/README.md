# 06 — Unsafe CPI Trust

**Bug:** The program performs CPI to an arbitrary program without validating the program ID or post‑conditions.  
**Impact:** A malicious program can execute unexpected behavior or corrupt state.

**Fix:** Restrict CPI program IDs and validate results/state after CPI.
