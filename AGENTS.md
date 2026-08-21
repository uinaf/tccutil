# tccutil-rs

Rust CLI for managing macOS TCC privacy permission databases. Single static binary; no runtime dependencies.

## Verify

```sh
make verify       # selective cached fmt + clippy + test (pre-push)
make verify-full  # forced CI gate (+ coverage + release build)
```

`make verify` runs independent stale lanes in parallel. Timestamp freshness is
an edit-loop optimization; use `make verify-full` after deleting or renaming
Rust sources and before claims that require exhaustive proof. CI always runs the
full gate.

## Build

```sh
cargo build --release
```

## Boundaries

- Prefer safe Rust; keep `unsafe` limited to the existing `geteuid` check
- System DB writes need `sudo`; SIP may still block on newer macOS
- Binary name stays `tccutil-rs` so it does not clash with Apple's `tccutil`

## Hooks

```sh
scripts/setup-hooks.sh
```

## Docs

- [README](README.md): install, commands, SIP limits
- [Contributing](CONTRIBUTING.md): setup, verify, pull requests
- [Releases](docs/RELEASES.md): Conventional Commits publish path
- [Security](SECURITY.md): private vulnerability reporting
