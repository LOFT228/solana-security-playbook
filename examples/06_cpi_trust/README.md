# 06 — Unsafe CPI Trust

**Bug:** Program accepts any CPI target without validation.

**Impact:** Malicious program can execute unexpected logic during CPI.

**Fix:** Validate CPI program ID against an allowlist or config.
