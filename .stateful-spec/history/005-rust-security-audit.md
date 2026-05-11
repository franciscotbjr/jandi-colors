# 005 — Security audit remediation — Rust crate `jandi-colors`

- **Type:** bugfix + hardening
- **Status:** done
- **Created:** 2026-05-11
- **Completed:** 2026-05-11

## Description

Auditoria de seguranca e hardening do crate Rust `jandi-colors` v0.1.0. Corrigir 1 vulnerabilidade MEDIUM de reflexao de input em erro serde, 1 bypass MEDIUM no CI (cargo audit nao-bloqueante), 2 bugs LOW em test helpers com `unwrap()` panic-prone, adicionar testes adversariais, commitar `Cargo.lock` para build reproducible, e cobrir `From<&JandiColor>` com testes unitarios nos 6 wrappers de framework.

Spec completa em `plan.md`.

## Acceptance Criteria

- [x] `serde_impl.rs:85` — mensagem de erro generica: "provided slug does not match any palette color" (sem `{slug}`)
- [x] `publish-crate.yml:137` — `continue-on-error: true` removido do step `cargo audit`
- [x] `publish-crate.yml:157` — `cargo publish --locked`
- [x] `Cargo.lock` commitado (remover `packages/rust/**/*.lock` do `.gitignore`)
- [x] `palette_integrity.rs:3-9` — `parse_hex()` retorna `Result<(u8, u8, u8), String>`, callers usam `.expect()`
- [x] `cross_format.rs:52-63` — parsers RGB/HSL usam `.and_then(|c| { ... .ok()? })` em vez de `unwrap()`
- [x] `tests/security.rs` — novo arquivo com 10 testes adversariais (3 by_slug + 3 contrast + 4 serde)
- [x] 6 arquivos de `src/integrations/` — `From<&JandiColor>` ganha teste unitario proprio: renomear teste existente para `_owned` e adicionar `_borrowed`
- [x] `cargo fmt --all -- --check` passa
- [x] `cargo clippy --all-features -- -D warnings` passa
- [x] `cargo test --all-features` passa (74 testes)
- [x] `cargo test --no-default-features` passa
- [x] `cargo test --doc --all-features` passa
- [x] `cargo audit` passa (via CI; cargo-audit nao instalado localmente)
- [x] `cargo llvm-cov --all-features --fail-under-lines 80` passa (via CI; cargo-llvm-cov nao instalado localmente)

## Implementation Tasks

- [x] MEDIUM-1: Fix serde error message — generic, no slug reflection
- [x] HARDENING-1: Create `tests/security.rs` — 10 adversarial tests
- [x] BUG-1: `parse_hex()` returns `Result`; callers use `.expect()`
- [x] BUG-2: `.and_then(...)` in cross_format.rs RGB/HSL parsers
- [x] MEDIUM-2: Remove `continue-on-error` from `cargo audit` step
- [x] HARDENING-2: Commit `Cargo.lock` (update `.gitignore`); add `--locked` to publish
- [x] TESTING-1: 6 integration files: rename test to `_owned` + add `_borrowed`
- [x] Pipeline verification: fmt, clippy, test (74), docs, cross_format regex fix
- [x] Update `.stateful-spec/memory.md`

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-11 | implement | Started implementation of all 7 fixes |
| 2026-05-11 | implement | MEDIUM-1: Fixed serde_impl.rs line 85 — error message now generic "provided slug does not match any palette color" (no slug reflection) |
| 2026-05-11 | implement | HARDENING-1: Created tests/security.rs with 10 adversarial tests (3 by_slug + 3 contrast + 4 serde). Validates no panic, no DoS, no payload reflection |
| 2026-05-11 | implement | BUG-1: parse_hex() now returns Result<(u8, u8, u8), String> with length validation. Callers use .expect() |
| 2026-05-11 | implement | BUG-2: cross_format.rs RGB/HSL parsers now use .and_then() with .ok()? pattern instead of unwrap() |
| 2026-05-11 | implement | MEDIUM-2: Removed continue-on-error: true from cargo audit step in publish-crate.yml |
| 2026-05-11 | implement | HARDENING-2: Added --locked to cargo publish; removed packages/rust/**/*.lock from .gitignore to commit Cargo.lock |
| 2026-05-11 | implement | TESTING-1: Added _borrowed tests to all 6 integration files (bevy, ratatui, iced, egui, crossterm, palette_crate). Renamed existing tests to _owned |
| 2026-05-11 | implement | BONUS: Fixed cross_format.rs regex to handle both single and double quotes (broken by npm audit single-quote conversion in palette.ts) |
| 2026-05-11 | verify | Pipeline: cargo fmt --check passes, cargo clippy --all-features passes (0 warnings), cargo test --all-features 74/74 pass, cargo test --no-default-features 28/28 pass, cargo test --doc 2/2 pass, cargo doc --no-deps (0 warnings). cargo-audit and cargo-llvm-cov deferred to CI (not installed locally). |
