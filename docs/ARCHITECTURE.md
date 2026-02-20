# Architecture

## Purpose
Provide a Rust replacement for macOS tccutil with full TCC DB management.

## Core Components
- CLI command layer for list/grant/revoke/reset flows.\n- TCC database access and mutation logic.\n- Output formatting for terminal and machine-readable modes.

## Domain Concepts
- macOS privacy services (TCC permissions).\n- User vs system database handling.

## System Boundaries
- This repo manages CLI behavior and DB operations only.\n- macOS UI and system policy behavior are external constraints.
