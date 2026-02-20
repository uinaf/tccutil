# AGENTS.md

Project: **tccutil**

## Scope
Project-specific agent instructions for this repo only.

## Start Here
- Read   -     README.md (human overview)
  - docs/ARCHITECTURE.md (high-level design)
  - docs/agents/PLAN.md (current plan)

## Tech Stack
- Rust CLI (single static binary)

## Workflow
- Keep diffs small and targeted.
- Prefer fixing root causes over patching symptoms.
- If behavior changes, update tests and docs together.
- Do not introduce placeholder sections in docs.

## Verify
- cargo fmt\n- cargo clippy -- -D warnings\n- cargo test

## Documentation Rules
- Keep README.md at project root.
- Keep CLAUDE.md as a symlink to AGENTS.md.
- Use docs/agents/ for working notes and planning.
