# Jandi Colors — Project Memory

> Stateful Spec entry point. Read this first when resuming work.

## Project

**Name:** Jandi Colors
**Package:** `@jandi/colors`
**Description:** Color palette of 8 design tokens derived from the blue pigment of jenipapo (Genipa americana), distributed as multi-format tokens (CSS, SCSS JSON W3C, Tailwind, Swift, Kotlin, Rust).

## Status

**Active development** — v0.1.0 (TypeScript npm + Rust crate)

## Open Session

- Currently open: `history/003-rust-cratesio-package.md`

## Active Work

- [x] Publicar jandi-colors como crate Rust (crates.io) → `history/003-rust-cratesio-package.md`

## Recent Completions

- [x] Stateful Spec onboarding and initial setup → `history/001-stateful-spec-onboarding.md` (2026-05-09)
- [x] Publicar @jandi/colors como pacote TypeScript npm → `history/002-npm-typescript-package.md` (2026-05-09)
- [x] Publicar jandi-colors como crate Rust (crates.io) → `history/003-rust-cratesio-package.md` (2026-05-10)

## Key Decisions

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-05-09 | Use Stateful Spec methodology | Improve AI-assisted development continuity |
| 2026-05-09 | Monorepo sem workspaces | `packages/typescript/` independente evita acoplamento |
| 2026-05-09 | Bundler tsup | Zero-config, ESM+CJS+.d.ts em um comando |
| 2026-05-09 | Assets estaticos duplicados + testes | Legibilidade direta, teste anti-drift |
| 2026-05-09 | source-of-truth.json dedicado | Consumivel por qualser linguagem |
| 2026-05-09 | RGB/HSL hardcoded + validacao matematica | Precisao garantida por teste |
| 2026-05-09 | CI automatizado com GitHub Actions | Pipeline completo com test + build + publish em tags |
| 2026-05-09 | ESLint + Prettier (strict) | Familiar, plugins maduros |

## Constraints & Reminders

- **Hex values must never change:** `#C8D5CC`, `#8FA7B3`, `#5D7F96`, `#3D5F80`, `#2C4A6E`, `#1E3452`, `#11203A`, `#0A1424`
- **Cultural names must not be altered** (Suco verde, Brisa, Oby, Jandi, Genipina, Nhandi, Yandi, Tinta de guerra)
- **Dual ESM/CJS package** via tsup
- **No runtime dependencies** — devDependencies only (tsup, vitest, typescript, eslint, prettier)
- **Scope `@jandi` deve ser criado manualmente no npmjs.com antes do primeiro publish**
- **NPM_TOKEN** deve ser adicionado como secret no repo
- **Rust crate:** `packages/rust/` — `#![no_std]`, feature flags for frameworks/serde/contrast
- **CARGO_REGISTRY_TOKEN** deve ser adicionado como secret no repo para publish do crate

## History Index

| # | Name | Status | Date |
|---|------|--------|------|
| 001 | Stateful Spec onboarding | done | 2026-05-09 |
| 002 | Publicar @jandi/colors como pacote TypeScript npm | done | 2026-05-09 |
| 003 | Publicar jandi-colors como crate Rust (crates.io) | in-progress | 2026-05-10 |

## Project Structure (after v0.1.0)

```
jandi-colors/
├── packages/typescript/     # @jandi/colors npm package
│   ├── src/               # TypeScript source
│   ├── css/               # CSS custom properties
│   ├── scss/             # SCSS variables + mixins
│   ├── json/              # W3C Design Tokens
│   ├── tailwind/           # Tailwind config
│   └── tests/             # vitest tests
├── packages/rust/          # jandi-colors (crates.io)
├── tokens/                # Ecosystems without package manager
│   ├── swift/
│   └── kotlin/
└── docs/
```