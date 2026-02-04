# 01 — Missing Authority Check

**Bug:** State-changing instruction does not verify that the caller is the stored authority.

**Impact:** Anyone can change config/vault state.

**Fix:** Require the caller to be a signer and match the stored authority (e.g., `has_one` or explicit key check).
