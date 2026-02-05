# 05 — State Mutation Before Validation

**Bug:** The program mutates state before all checks are complete.  
**Impact:** Partial state changes can occur even if later validation fails.

**Fix:** Validate inputs and invariants first, mutate state last.
