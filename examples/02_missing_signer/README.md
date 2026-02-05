# 02 — Missing Signer Requirement

**Bug:** The correct authority account is passed, but the program never requires it to sign.  
**Impact:** Anyone can call the instruction without the authority’s private key.

**Fix:** Mark the authority as `Signer` (Anchor) or check `is_signer` manually.
