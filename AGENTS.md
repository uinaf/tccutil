# tccutil-rs

Rust CLI for managing macOS TCC privacy permission databases. Single static binary; no runtime dependencies.

## Verify

```sh
scripts/verify.sh          # fmt + clippy + test (pre-push)
scripts/verify.sh --full   # CI gate (+ coverage + release build)
```

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

- [README](README.md) — install, commands, SIP limits
- [Contributing](CONTRIBUTING.md) — setup, verify, pull requests
- [Releases](docs/RELEASES.md) — Conventional Commits publish path
- [Security](SECURITY.md) — private vulnerability reporting
