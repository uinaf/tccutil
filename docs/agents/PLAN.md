# Project Plan

**Goal:** Keep agent instructions and documentation aligned with real project behavior.
**Approach:** Apply small, reviewable updates; verify with native project commands before PRs.
**Tech:** Rust CLI (single static binary)

---

### Task 1: Keep docs aligned with implementation

**Files:**
- Modify: README.md, docs/ARCHITECTURE.md, AGENTS.md

**Steps:**
1. Read current README and scripts/tooling config.
2. Update docs only with verifiable facts from the repo.
3. Run project checks.
4. Open/refresh PR with scoped doc+agent changes.

**Verify:**
- cargo fmt\n- cargo clippy -- -D warnings\n- cargo test
