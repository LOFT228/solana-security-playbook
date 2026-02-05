# 01 — Missing Authority Check

**Bug:** The program updates a config account without verifying that the caller matches the stored authority.  
**Impact:** Anyone can take over or change sensitive configuration.

**Fix:** Enforce `has_one = authority` and require the authority to be a signer.
