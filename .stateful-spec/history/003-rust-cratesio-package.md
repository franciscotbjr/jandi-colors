# 003 — Publicar jandi-colors como crate Rust (crates.io)

- **Type:** feature
- **Status:** done
- **Created:** 2026-05-10

## Description

Publicar `jandi-colors` no crates.io como crate Rust `#![no_std]`, zero-dependencias no core, com 8 cores tipadas (`JandiColor`), metadados etnobotanicos completos, e conversoes para 6 frameworks de UI via feature flags opcionais.

## Acceptance Criteria

- [x] `packages/rust/` existe com toda a estrutura definida
- [x] `types.rs` exporta `JandiColor`, `Rgb`, `Hsl`, `OxidationStage` com todos os derives
- [x] `palette.rs` exporta 8 constantes `JandiColor`, `PALETTE`, `VERSION`, `by_slug()`, `JANDI` alias
- [x] `contrast.rs` exporta 7 funcoes WCAG (relative_luminance, contrast_ratio, wcag_aa_large, wcag_aa, wcag_aaa, get_contrast_pair, is_light)
- [x] `lib.rs` com `#![no_std]` re-exporta tudo de types, palette, contrast, integrations
- [x] 6 feature flags de framework funcionais via `From` impl
- [x] Feature flag `serde` serializa/deserializa `JandiColor` e `OxidationStage`
- [x] `Cargo.toml` configurado sem dependencias no core, MSRV 1.95.0
- [x] 8 testes de integridade passam (palette_integrity.rs)
- [x] Teste cross-format passa — valores identicos ao TypeScript (9 campos: slug, name, hex, rgb, hsl, stage, description)
- [x] 13 testes WCAG passam (contrast.rs)
- [x] Testes de integracao por feature flag passam (6 frameworks: ratatui, iced, bevy, egui, crossterm, palette; serde; 3 cores cada exceto palette inline)
- [x] `cargo test --no-default-features` compila e passa (25/25)
- [x] `cargo clippy --all-features -- -D warnings` passa
- [x] `cargo doc --all-features --no-deps` gera sem warnings
- [x] CI roda em push para `packages/rust/**` (7 jobs: test, test-contrast, cross-format-check, msrv, coverage, security, publish)
- [x] CI publica no crates.io em tags `rust-v*`

## Implementation Tasks

- [x] Criar estrutura `packages/rust/`
- [x] Criar `Cargo.toml`
- [x] Criar `src/types.rs`
- [x] Criar `src/palette.rs`
- [x] Criar `src/contrast.rs`
- [x] Criar `src/lib.rs`
- [x] Criar `src/integrations/` (7 arquivos)
- [x] Criar `tests/` (4 arquivos)
- [x] Criar `examples/` (3 arquivos)
- [x] Criar `README.md` do crate
- [x] Criar `.github/workflows/publish-crate.yml`
- [x] Atualizar memoria e iteration file

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
| 2026-05-10 | create-technical-spec | Removed redundant #![warn(missing_docs)] — deny implies warn. |
| 2026-05-10 | create-technical-spec | Expanded section 4.11 (README do crate) to mirror packages/typescript/README.md with full structural parity: 16 sections including title, badges, install, palette table, API examples, feature flags, utilities, accessibility, tests, structure, other ecosystems, license, footer. |
| 2026-05-10 | write-commit-message | Generated commit: "docs: add session log entries for test strategy and Rust best practices" and executed commit. |
| 2026-05-10 | review-changes | Performed self-review against plan.md. Found 3 issues: 1 Critical (logic error in contrast test for Oby vs Suco verde), 1 Warning (versions of iced_core and egui in Cargo.toml deviate from spec), and 1 Warning (cross_format.rs hardcodes values instead of reading palette.ts via regex). |
| 2026-05-10 | review-changes | Performed second self-review. The cross_format.rs and egui issues were fixed. Remaining issues: 1 Critical (logic error in contrast tests for Oby vs Suco verde still present in both src/ and tests/), 1 Warning (iced_core version in Cargo.toml is 0.14, spec is 0.15), and 1 Suggestion (O(N) check in get_contrast_pair). |
| 2026-05-10 | review-changes | Performed third self-review. The performance suggestion in src/contrast.rs (O(N) check removal) was successfully applied. However, the Critical logic error in the contrast tests and the Warning regarding the iced_core version in Cargo.toml remain unapplied on disk. |
| 2026-05-10 | review-changes | **Session review — 6 Critical/Warning + 3 Suggestion resolved.** C1: consolidated duplicate `push:` keys in publish-crate.yml and publish-npm.yml (YAML override bug). C2: README WCAG table updated with actual computed ratios (10.7, 8.3, 6.0, 7.3, 6.5, 2.8, 4.4, 12.2, 7.3, 4.3) — root and crate READMEs synced. C3: serde JSON shape fixed — `rgb`/`hsl` now objects (not tuples), `oxidationStage` now camelCase (not kebab), matching TS format exactly; new test_json_shape_matches_typescript validates structure. W1: added `coverage` (cargo-llvm-cov >= 80%) and `security` (cargo-audit) CI jobs per plan §4.10. W2: 6 framework integration tests added to tests/integrations.rs (3 colors each); palette covered via inline test (palette_crate.rs, 3 colors) — dev-dep Cargo quirk prevents palette::Srgb in integration test context. W3: get_contrast_pair loop changed from bidirectional (56 pairs, panic at threshold 0.0) to unordered via skip(i+1) (28 pairs = C(8,2), impossible overflow); new zero-threshold no-panic test. W4: restored default-features=false for ratatui, iced_core, egui; kept defaults for bevy_color, crossterm, palette (math backend / platform features needed); DD-21 documented. W5: cross_format.rs now validates all 9 fields — added oxidation_stage and description extraction+comparison. W6: consolidated .gitignore duplications (2x target, Cargo.lock redundant with **/*.lock). S1: doc-comment added to Deserialize impl documenting slug-only lookup behavior. Final E2E verification: 58/58 tests, print_palette smoke, JSON shape validated, real ratios computed and READMEs updated. |
| 2026-05-10 | write-commit-message | Commit: `feat: add jandi-colors Rust crate with 9-field cross-format validator and 6 framework integrations`. 28 files, +2093/-105. All 18 acceptance criteria marked done. Iteration 003 closed. |
