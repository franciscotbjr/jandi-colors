# 003 — Publicar jandi-colors como crate Rust (crates.io)

- **Type:** feature
- **Status:** in-progress
- **Created:** 2026-05-10

## Description

Publicar `jandi-colors` no crates.io como crate Rust `#![no_std]`, zero-dependencias no core, com 8 cores tipadas (`JandiColor`), metadados etnobotanicos completos, e conversoes para 6 frameworks de UI via feature flags opcionais.

## Acceptance Criteria

- [ ] `packages/rust/` existe com toda a estrutura definida
- [ ] `types.rs` exporta `JandiColor`, `Rgb`, `Hsl`, `OxidationStage` com todos os derives
- [ ] `palette.rs` exporta 8 constantes `JandiColor`, `PALETTE`, `VERSION`, `by_slug()`, `JANDI` alias
- [ ] `contrast.rs` exporta 5 funcoes WCAG
- [ ] `lib.rs` com `#![no_std]` re-exporta tudo de types, palette, contrast, integrations
- [ ] 6 feature flags de framework funcionais via `From` impl
- [ ] Feature flag `serde` serializa/deserializa `JandiColor` e `OxidationStage`
- [ ] `Cargo.toml` configurado sem dependencias no core, MSRV 1.70
- [ ] 5 testes de integridade passam
- [ ] Teste cross-format passa — valores identicos ao TypeScript
- [ ] 10 testes WCAG passam
- [ ] Testes de integracao por feature flag passam
- [ ] `cargo test --no-default-features` compila
- [ ] `cargo clippy --all-features -- -D warnings` passa
- [ ] `cargo doc --all-features --no-deps` gera sem warnings
- [ ] CI roda em push para `packages/rust/**`
- [ ] CI publica no crates.io em tags `rust-v*`

## Implementation Tasks

- [ ] Criar estrutura `packages/rust/`
- [ ] Criar `Cargo.toml`
- [ ] Criar `src/types.rs`
- [ ] Criar `src/palette.rs`
- [ ] Criar `src/contrast.rs`
- [ ] Criar `src/lib.rs`
- [ ] Criar `src/integrations/` (7 arquivos)
- [ ] Criar `tests/` (4 arquivos)
- [ ] Criar `examples/` (3 arquivos)
- [ ] Criar `README.md` do crate
- [ ] Criar `.github/workflows/publish-crate.yml`
- [ ] Atualizar memoria e iteration file

## Decisions

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-05-10 | Core `#![no_std]` sem dependencias | Custo de compilacao base zero, embedded/WASM |
| 2026-05-10 | Feature flags com `From<JandiColor>` | Idiomatico Rust, `.into()` sem import extra |
| 2026-05-10 | JandiColor como struct, nao enum | Acesso direto (`PRIMARY.hex`) sem match |
| 2026-05-10 | Sub-crates para deps de framework | Reduz grafo de compilacao drasticamente |
| 2026-05-10 | Sem build script | 8 cores, valores estaveis |
| 2026-05-10 | Alinhamento TS com teste cross-format | Source of truth e o TypeScript |
| 2026-05-10 | Feature `colored` adiado para v0.2 | Reduz scope de v0.1 |

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-10 | create-technical-spec | Created technical specification for jandi-colors Rust crate. Extracted from prompt.md into formal plan.md format with 8 sections: summary, complexity analysis, dependencies, detailed spec (12 subsections), acceptance criteria (18 items), decision log (14 decisions), risks (7 items), open questions (4 items). |
| 2026-05-10 | create-technical-spec | Updated plan: changed Cargo.toml edition to 2024 and MSRV to 1.95.0. |
| 2026-05-10 | create-technical-spec | Added DD-15: css-colors feature intentionally omitted — JandiColor fields are already public, adding helpers would break no_std without new capability. |
| 2026-05-10 | create-technical-spec | Added DD-16: contrast.rs as superset of TS utils.ts — building blocks public (idiomatic Rust), WCAG predicates, get_contrast_pair/is_light for TS parity. Updated section 4.4 with full API table. |
| 2026-05-10 | create-technical-spec | Added DD-17: contrast as feature flag with libm + arrayvec. f32::powf not stabilized in core/no_std through Rust 1.95. Feature flag preserves zero-deps core promise. Updated sections 4.4, 4.5 (lib.rs gating), 4.7 (Cargo.toml), 4.8 (tests), 4.10 (CI job), 5 (acceptance criteria). |
| 2026-05-10 | create-technical-spec | Removed tokens/rust/ from plan structure — Rust has its own package at packages/rust/, tokens/ is only for ecosystems without a dedicated package (swift, kotlin). |
| 2026-05-10 | create-technical-spec | Added DD-18: tokens/rust/ removed — discontinue copy-paste channel, require cargo add. Avoiding API drift between crate and raw token. Added acceptance criterion for git rm. Added scope note clarifying tree is partial view (other tokens/ subdirs are TS assets, out of scope). |
| 2026-05-10 | create-technical-spec | Updated print_palette.rs to use crossterm for ANSI true-color (honoring prompt.md:82). Added [[example]] sections to Cargo.toml for all 3 examples. Added acceptance criterion for example compilation. |
| 2026-05-10 | create-technical-spec | Cosmetic refinements: updated summary to include serde+contrast features, added serde+contrast to README feature flag examples, added libm version drift risk to risks table. |
| 2026-05-10 | write-commit-message | Generated commit message: "feat: add Rust crate spec (003) with DD-15 through DD-18" and executed commit. |
| 2026-05-10 | create-technical-spec | Expanded test strategy to 3 layers: unit tests inline (types.rs, palette.rs, contrast.rs, lib.rs), integration tests (8 palette + 3 per framework), doc-tests, coverage gate >= 80% via cargo llvm-cov. Added DD-19. Updated CI with doc-test and coverage jobs. |
| 2026-05-10 | create-technical-spec | Closed 3 test gaps: added serde_impl.rs unit tests + integration tests, fixed doc-tests to use --all-features, added MSRV verification job, cargo fmt --check, #![deny(missing_docs)], cargo audit. Added DD-20. Coverage estimate: 85-90%. |
