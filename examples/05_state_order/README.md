# 05 — State Mutation Before Validation

**Bug:** Program mutates state before checking constraints.

**Impact:** State can be corrupted if validation fails or if a CPI re-enters.

**Fix:** Validate everything first, then mutate state.
