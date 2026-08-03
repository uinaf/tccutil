# Releases

Pushes to `main` release automatically. Skip a push with `[skip ci]`.

## Versioning

Conventional Commits drive the bump (see `.releaserc.json`):

| Commit type | Release |
|---|---|
| `feat:` | minor |
| `fix:` / `perf:` | patch |
| `feat!:` / breaking change | major |
| `docs:` / `test:` / `chore:` / `refactor:` / `build:` / `ci:` | none |

## Pipeline

1. `verify` runs with read-only credentials
2. Protected `release` Environment mints a short-lived `uinaf-releaser` installation token scoped to `tccutil-rs`
3. `semantic-release` bumps Cargo files, creates the GitHub Release, and the job attaches darwin archives
4. A follow-up job mints a Contents-only token scoped to `tccutil-rs` + `homebrew-tap` and updates the Homebrew formula

Sources of truth: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml), [`.releaserc.json`](../.releaserc.json).

## Credentials

`release` Environment:

| Name | Kind |
|---|---|
| `UINAF_RELEASE_APP_ID` | variable |
| `UINAF_RELEASE_APP_PRIVATE_KEY` | secret |
