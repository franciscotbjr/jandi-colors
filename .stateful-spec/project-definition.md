# Project Definition — Jandi Colors

## 1. Project Identity

- **Name:** Jandi Colors
- **npm package:** `@jandi/colors` v0.1.0
- **Description:** Color palette of 8 design tokens, derived from the blue pigment of jenipapo (Genipa americana), distributed as multi-format tokens (CSS, SCSS, JSON, Tailwind, Swift, Kotlin, Rust)
- **Repository:** `https://github.com/franciscotbjr/jandi-colors`
- **License:** MIT
- **Author:** Francisco Tarcizo Bomfim Junior (Chico)

## 2. Technology Stack

- **Primary:** Node.js (CommonJS), no runtime dependencies
- **Token outputs:** CSS custom properties, SCSS variables, JSON (W3C format), Tailwind CSS plugin, Swift (SwiftUI/UIKit), Kotlin (Jetpack Compose), Rust (consts)
- **Publish targets:** npm (`@jandi/colors`) + cargo (Rust crate)

## 3. Repository Structure

```
index.js              → Entry point (CommonJS): hex, rgb, hsl + named exports
tokens/
  css/jandi.css       → CSS custom properties + semantic aliases + alpha
  scss/_jandi.scss    → SCSS variables + mixins
  json/jandi.tokens.json → W3C Design Tokens format
  tailwind/jandi.config.js → Tailwind CSS plugin
  swift/JandiColors.swift → SwiftUI / UIKit
  kotlin/JandiColors.kt   → Jetpack Compose / Android
  rust/jandi.rs           → Rust const
docs/
  assets/              → palette-banner.svg
  chemistry/           → jenipapo chemistry docs
CONTRIBUTING.md        → Contribution rules
```

## 4. Code Conventions

- **Module system:** CommonJS (`module.exports` / `require`)
- **Naming:** camelCase (JS), kebab-case (CSS classes/Tailwind), snake_case (SCSS vars)
- **Documentation:** Inline comments in Portuguese/English in token files
- **No formatter/linter config** currently present

## 5. Testing Strategy

- **Current:** No tests
- **Proposed:** Validate consistency of all 8 hex values across all token formats

## 6. Quality Gates

- **Current:** No CI/quality gates defined
- **Minimum proposal:** Script that verifies all token files contain the same 8 hex values

## 7. Constraints & Non-Negotiables

- The **8 hex values of the palette must never change:**
  - `#C8D5CC`, `#8FA7B3`, `#5D7F96`, `#3D5F80`, `#2C4A6E`, `#1E3452`, `#11203A`, `#0A1424`
- The **cultural names of the tones** (Suco verde, Brisa, Oby, Jandi, Genipina, Nhandi, Yandi, Tinta de guerra) must not be altered
- **Publish targets:** npm + cargo
