# 002 — Publicar @jandi/colors como pacote TypeScript npm

- **Type:** feature
- **Status:** done
- **Created:** 2026-05-09
- **Completed:** 2026-05-09

## Description

Publicar `@jandi/colors` no npm registry como pacote TypeScript-first, dual ESM/CJS, zero-dependencias, com API tipada rica e assets estaticos co-distribuidos.

## Acceptance Criteria

- [x] `packages/typescript/` existe com toda a estrutura
- [x] `source-of-truth.json` contem os 8 hex values, usado como fonte unica de verdade
- [x] `palette.ts` exporta 8 constantes `JandiColor` + `palette: JandiPalette`
- [x] `utils.ts` exporta `hexToRgb`, `hexToHsl`, `getContrastPair`, `isLight`
- [x] `index.ts` re-exporta tudo de types, palette, utils
- [x] CSS, SCSS, JSON W3C, Tailwind enriquecidos com metadados completos
- [x] `package.json` com dual ESM/CJS exports map, scripts, devDependencies
- [x] `tsup.config.ts`, `tsconfig.json`, ESLint, Prettier configurados
- [x] `npm run build` gera `dist/` com ESM + CJS + `.d.ts`
- [x] `npm run typecheck` passa sem erros
- [x] `npm run lint` passa sem erros
- [x] `npm run format` passa sem erros
- [x] 8 testes de integridade cross-formato passam
- [x] 10 testes WCAG passam
- [x] CI roda em push para `packages/typescript/**`
- [x] CI publica npm em tags `v*`
- [x] Raiz limpa: `package.json` e `index.js` removidos
- [x] `tokens/` na raiz contem apenas swift/, kotlin/, rust/

## Implementation Tasks

- [x] Criar estrutura `packages/typescript/`
- [x] Criar `source-of-truth.json`
- [x] Criar `src/types.ts`
- [x] Criar `src/palette.ts` com 8 cores + metadados
- [x] Criar `src/utils.ts` com funcoes de conversao
- [x] Criar `src/index.ts` re-export
- [x] Criar `css/jandi.css` enriquecido
- [x] Criar `scss/_jandi.scss` enriquecido
- [x] Criar `json/jandi.tokens.json` W3C
- [x] Criar `tailwind/jandi.config.js`
- [x] Criar `package.json` com dual ESM/CJS
- [x] Criar `tsconfig.json`
- [x] Criar `tsup.config.ts`
- [x] Configurar ESLint + Prettier
- [x] Criar `tests/palette.test.ts`
- [x] Criar `tests/contrast.test.ts`
- [x] Criar `.github/workflows/publish-npm.yml`
- [x] Criar `README.md` do pacote
- [x] Remover `package.json` e `index.js` da raiz
- [x] Atualizar memoria e iteration file

## Decisions

| Date | Decision | Rationale |
|------|----------|-----------|
| 2026-05-09 | Monorepo sem workspaces | `packages/typescript/` independente evita acoplamento |
| 2026-05-09 | Bundler tsup | Zero-config, ESM+CJS+.d.ts |
| 2026-05-09 | Assets estaticos duplicados + testes | Legibilidade direta |
| 2026-05-09 | source-of-truth.json dedicado | Consumivel por qualquer linguagem |
| 2026-05-09 | RGB/HSL hardcoded + validacao matematica | Precisao garantida por teste |
| 2026-05-09 | CI automatizado | Pipeline completo com publish em tags |
| 2026-05-09 | ESLint + Prettier | Familiar, plugins maduros |

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-09 17:53 | create-technical-spec | Spec created after exhaustive questioning |
| 2026-05-09 | implementation | Created packages/typescript/ structure |
| 2026-05-09 | implementation | Created TypeScript source files (types, palette, utils, index) |
| 2026-05-09 | implementation | Created enriched static assets (CSS, SCSS, JSON, Tailwind) |
| 2026-05-09 | implementation | Created config files (package.json, tsconfig, tsup, ESLint, Prettier) |
| 2026-05-09 | implementation | Created tests (palette.test.ts, contrast.test.ts) |
| 2026-05-09 | implementation | Created CI workflow |
| 2026-05-09 | implementation | Created package README |
| 2026-05-09 | implementation | Removed root package.json and index.js |
| 2026-05-09 | update-documentation | Updated memory.md and created iteration file |
| 2026-05-09 18:38 | review-changes | Code review performed. Critical bugs found in utils.ts (getLuminance) and palette.test.ts (HSL conversion), plus a warning for eslint.config.ts. |
| 2026-05-09 19:35 | review-changes | Re-review performed. Correctness issues (tests/types) resolved. ESLint warnings persist and evolved into parser/env errors. |
| 2026-05-09 19:46 | review-changes | Final re-review performed. All ESLint config and parsing errors resolved. Code review is fully clean. |
| 2026-05-09 19:54 | write-commit-message | Generated commit message for publishing @jandi/colors as TypeScript npm package and executed commit. |
| 2026-05-09 19:57 | save-session | Progress saved. All tests, linting, and configurations verified and committed. |

## Notes

- **Scoped NPM `@jandi`** precisa ser criado manualmente em https://www.npmjs.com/org/create antes do primeiro publish
- **NPM_TOKEN** precisa ser adicionado como secret no repo GitHub Settings > Secrets
- Os 8 hex values nunca devem mudar — os testes validam isso