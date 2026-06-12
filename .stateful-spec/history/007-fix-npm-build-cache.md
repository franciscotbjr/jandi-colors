# 007 — Fix npm build cache (GitHub Actions)

- **Type:** bugfix
- **Status:** in-progress
- **Created:** 2026-06-12

## Description

O workflow `build-npm.yml` falha no GitHub Actions com erro "Some specified paths were not resolved, unable to cache dependencies" no step `actions/setup-node@v4`. Causa raiz: `packages/typescript/package-lock.json` está no `.gitignore` e não é commitado.

## Root Cause

`packages/typescript/package-lock.json` in `.gitignore:20` → `actions/setup-node@v4` com `cache: npm` + `cache-dependency-path: packages/typescript/package-lock.json` não encontra o arquivo. Além disso, `npm ci` (step seguinte) também falharia sem o lockfile.

## Acceptance Criteria

- [ ] `package-lock.json` removido do `.gitignore`
- [ ] `package-lock.json` gerado e commitado no repositório
- [ ] `build-npm.yml` deve encontrar o `cache-dependency-path` após checkout

## Implementation Tasks

- [ ] Remover linha `packages/typescript/package-lock.json` do `.gitignore`
- [ ] Executar `npm install` em `packages/typescript/` para gerar o lockfile
- [ ] Verificar que `package-lock.json` foi criado

## Decisions

| # | Decision | Rationale |
|---|----------|-----------|
| D-1 | Versionar `package-lock.json` | Prática padrão para builds reproduzíveis em CI |

## Session Log

| Timestamp | Operation | Summary |
|-----------|-----------|---------|
| 2026-06-12 | analyze | Identified root cause: package-lock.json in .gitignore prevents cache resolution and npm ci |
| 2026-06-12 | plan | 3-step fix: remove from gitignore, npm install, commit lockfile |
