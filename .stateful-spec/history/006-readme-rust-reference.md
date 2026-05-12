# 006 — Adicionar referência Rust ao README.md raiz

- **Type:** documentation
- **Status:** done
- **Created:** 2026-05-11
- **Completed:** 2026-05-11

## Description

Atualizar o README.md raiz para incluir referência à implementação Rust, seguindo o modelo da referência TypeScript já existente. Incluir links para os READMEs de ambos os pacotes e atualizar o bloco "Tokens disponíveis" para refletir a estrutura do monorepo.

Além disso, revisar os READMEs internos dos pacotes (TypeScript e Rust) para corrigir inconsistências, paths desatualizados, e cross-links quebrados. Por fim, unificar a estrutura de ambos os READMEs (idioma EN, seções na mesma ordem, badges).

## Acceptance Criteria

- [x] `## Instalação` inclui `### Rust` com `cargo add` + exemplo básico
- [x] `### npm` ganha link para `packages/typescript/README.md`
- [x] `### Rust` ganha link para `packages/rust/README.md`
- [x] `## Tokens disponíveis` substituído por tabela com 2 linhas (TypeScript + Rust)
- [x] Nenhum conteúdo existente do README é removido ou alterado indevidamente
- [x] Root README: paths `/tokens/css/` e `/tokens/scss/` corrigidos
- [x] Root README: `import { colors }` → `import { palette }`
- [x] TypeScript README: "38 testes" → "55 testes"
- [x] TypeScript README: tabela de acessibilidade sincronizada com Rust/root (10 linhas, ratios corretos)
- [x] TypeScript README: cross-link corrigido para apontar para `../rust/README.md`
- [x] TypeScript README: SCSS import path corrigido (`scss/jandi` → `scss`)
- [x] Rust README: link npm agora inclui `../typescript/README.md`
- [x] Rust README: Swift/Kotlin descritos como raw token files com paths corretos
- [x] TypeScript README: reescrito em EN com badges (npm version, bundle size, license, TypeScript)
- [x] TypeScript README: seções reordenadas — `## Installation` → `## The Palette` → `## Usage` → `## Accessibility` → `## Running Tests` → `## Package Structure` → `## Other Ecosystems` → `## License`
- [x] TypeScript README: `## API TypeScript` + `## Utilitários` fundidos em `## Usage`
- [x] TypeScript README: "Estágio" → "Oxidation Stage" na tabela da paleta (alinhado com Rust)
- [x] TypeScript README: todos os títulos de seção traduzidos para EN
- [x] TypeScript README: `jandi.config.js` → `jandi.config.cjs` no tree de estrutura
- [x] TypeScript README: footer traduzido ("From fruit to screen.")
- [x] Rust README: `## Basic API (zero dependencies)` → `## Usage`
- [x] Ambos os READMEs com mesma sequência de seções (exceto Feature Flags só no Rust)

## Decisions

| # | Decision |
|---|----------|
| D-1 | Escopo mínimo para seção Rust: cargo add + exemplo básico |
| D-2 | Links para READMEs dentro de cada subseção (`### npm`, `### Rust`) |
| D-3 | Bloco Tokens disponíveis atualizado com paths do monorepo |
| D-4 | Formato simples: tabela com TypeScript + Rust, sem detalhar formatos internos |
| D-5 | Idioma unificado: EN para ambos os pacotes |
| D-6 | Badges TS: npm version + bundle size (BundlePhobia) + license (MIT) + TypeScript 5.8 |
| D-7 | Feature Flags só no Rust; TypeScript omite a seção |

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-11 | start-session | Created iteration file. Plan defined with 4 decisions. |
| 2026-05-11 | implement | Root README: added `### Rust` subsection, npm/Rust README links, replaced `## Tokens disponíveis` with `## Ecossistemas` table |
| 2026-05-11 | verify | Review of internal package READMEs found 6 issues (3 bugs in root, 4 in TypeScript, 2 in Rust) |
| 2026-05-11 | plan | Planned 9 corrections across 3 files |
| 2026-05-11 | implement | Root README: fixed `/tokens/css/` path, `{ colors }` import, `/tokens/scss/` path |
| 2026-05-11 | implement | TypeScript README: fixed SCSS import, test count (38→55), accessibility table (4→10 rows), cross-link |
| 2026-05-11 | implement | Rust README: added TypeScript README link, clarified Swift/Kotlin as raw token files |
| 2026-05-11 | implement | Fixed relative paths: all cross-links between package READMEs use `../` (GitHub-compatible) |
| 2026-05-11 | analyze | Package README review — identified structural divergence (order, language, badges, section naming) |
| 2026-05-11 | plan | Designed unified structure: 10-section template, EN for both, badges for TS, Feature Flags only in Rust |
| 2026-05-11 | implement | TypeScript README: full rewrite in EN with 4 badges, reordered sections, merged API+Utils into Usage |
| 2026-05-11 | implement | Rust README: renamed `## Basic API` → `## Usage` (minimal change) |
| 2026-05-11 | verify | Confirmed both READMEs have identical `## ` section sequence (except Feature Flags in Rust only) |
