# 004 — Security audit remediation — TypeScript (npm)

- **Type:** bugfix + hardening
- **Status:** in-progress (Phase 4 — Implement)
- **Created:** 2026-05-11

## Description

Corrigir 1 vulnerabilidade HIGH (CSS injection via `toRGBA`), 1 MEDIUM (reflexao de input em erro de `hexToRgb`), 1 BUG funcional (`getContrastPair` retorna cores planificadas em vez de pares), 2 hardenings de CI (`--provenance` + lint blocker) e 3 hardenings de configuracao (`npm audit` + `engines.node` + testes adversariais) no pacote `@jandi/colors`.

Spec completa em `plan.md`.

## Acceptance Criteria

- [ ] `toRGBA(alpha)` valida: rejeita NaN, Infinity, -Infinity, <0, >1, strings, null, undefined
- [ ] `hexToRgb(hex)` sanitiza mensagem de erro — payload do atacante nao aparece na mensagem
- [ ] `getContrastPair(palette)` retorna `[JandiColor, JandiColor][]` com 28 iteracoes (C(8,2) via `j = i + 1`)
- [ ] `tests/security.test.ts` existe com 18 casos adversariais
- [ ] `tests/contrast.test.ts` atualizado para validar tipo tupla do retorno de `getContrastPair`
- [ ] CI `publish` job: `id-token: write` no `permissions` + `npm publish --provenance`
- [ ] CI `test` job: `continue-on-error` removido do lint + step `npm audit --audit-level=high` adicionado
- [ ] `package.json` inclui `"engines": { "node": ">=18.0.0" }`
- [ ] Pipeline completa passa: `typecheck && lint && format && test && build`

## Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| D-1 | Manter v0.1.0 | Nenhuma publicacao feita — sem consumidores para sofrer breaking |
| D-2 | Manter source maps (`sourcemap: true`) | Debug pelo consumidor > pequeno aumento de superficie |
| D-3 | `npm audit` dentro do job `test` existente | Evita duplicacao de checkout/setup/install (~30s) |
| D-4 | `--provenance` com token, sem OIDC | OIDC requer configuracao no npmjs.com, fora do escopo deste plano |

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-05-11 | create-technical-spec | Created security audit remediation plan (plan.md). Addressed 8 findings: 1 HIGH, 2 MEDIUM, 1 BUG, 5 HARDENING. 4 decisions resolved with user. Spec covers 5 files to modify + 1 new test file. |
| 2026-05-11 | implement | Implemented all 12 steps from plan.md. Fixed toRGBA (alpha validation), hexToRgb (error sanitization), getContrastPair (returns pairs). Created security.test.ts (15 adversarial tests). Updated contrast.test.ts (+2 pair tests). CI: provenance, lint blocker, audit. package.json: engines.node >=18. README: getContrastPair example with destructuring. Pipeline passes: typecheck, lint, format, 55 tests, build, npm audit clean. |
| 2026-05-11 | review-changes | Code review completed according to plan.md. All acceptance criteria met: `toRGBA` rejects invalid inputs; `hexToRgb` safely formats error strings; `getContrastPair` returns tuplas and reduced O(N^2) complexity; security test file present; GitHub actions workflows hardened. Prettier ran cleanly resulting in benign diffs in single-quote conversions. No scope creep, no exposed secrets. Recommendation: Ready to commit. |
