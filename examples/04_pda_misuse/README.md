# 04 — PDA Misuse (Seeds / Bump Not Enforced)

**Bug:** The program assumes a PDA is correct without verifying its seeds/bump.  
**Impact:** Attackers can pass their own account instead of the intended PDA.

**Fix:** Use Anchor `seeds`/`bump` constraints or verify PDA with `find_program_address`.
