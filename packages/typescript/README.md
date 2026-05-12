# @jandi/colors

**A color palette derived from the blue pigment of jenipapo (_Genipa americana_), the ancestral dye of Brazil's indigenous peoples.**

<p align="center">
  <img src="https://raw.githubusercontent.com/franciscotbjr/jandi-colors/main/docs/assets/palette-banner.svg" alt="Jandí color palette" width="400">
</p>

[![npm version](https://img.shields.io/npm/v/@jandi/colors)](https://www.npmjs.com/package/@jandi/colors)
[![bundle size](https://img.shields.io/bundlephobia/min/@jandi/colors)](https://bundlephobia.com/package/@jandi/colors)
[![license](https://img.shields.io/badge/license-MIT-green)](https://github.com/franciscotbjr/jandi-colors/blob/main/LICENSE)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.8-blue)](https://www.typescriptlang.org/)

---

## Installation

```bash
npm install @jandi/colors
```

### npm

```js
import { palette, jandi, oby, genipina } from '@jandi/colors'
```

### CSS

```html
<link rel="stylesheet" href="https://unpkg.com/@jandi/colors/css/jandi.css">
```

```css
.element {
  background: var(--jandi-primary);
  color: var(--jandi-suco-verde);
}
```

### Tailwind

```js
// tailwind.config.js
const { jandiColors } = require('@jandi/colors/tailwind')

module.exports = {
  theme: {
    extend: {
      colors: jandiColors,
    },
  },
}
```

```html
<div class="bg-jandi-primary text-jandi-brisa">...</div>
```

### SCSS

```scss
@import '@jandi/colors/scss';

.element {
  background: $jandi-primary;
  color: $jandi-text-primary;
}
```

> See the [monorepo root](https://github.com/franciscotbjr/jandi-colors) for all available formats.

## The Palette

Eight tones derived from the real behavior of genipin pigment, from fresh green pulp to concentrated ritual war paint.

| Name | Hex | RGB | HSL | Oxidation Stage |
|------|-----|-----|-----|-----------------|
| **Suco verde** | `#C8D5CC` | 200, 213, 204 | 138°, 13%, 81% | Pre |
| **Brisa** | `#8FA7B3` | 143, 167, 179 | 200°, 19%, 63% | Initial |
| **Oby** | `#5D7F96` | 93, 127, 150 | 204°, 23%, 48% | Partial |
| **Jandí** | `#3D5F80` | 61, 95, 128 | 210°, 35%, 37% | Full |
| **Genipina** | `#2C4A6E` | 44, 74, 110 | 213°, 43%, 30% | Saturated |
| **Nhandí** | `#1E3452` | 30, 52, 82 | 215°, 46%, 22% | Deep |
| **Yandí** | `#11203A` | 17, 32, 58 | 218°, 55%, 15% | Concentrated |
| **Tinta de guerra** | `#0A1424` | 10, 20, 36 | 217°, 57%, 9% | Max |

## Usage

```typescript
import { palette, primary, sucoVerde, hexToRgb, hexToHsl, isLight, getContrastPair } from '@jandi/colors'

// Color access
console.log(primary.hex)        // "#3D5F80"
console.log(primary.rgb)        // { r: 61, g: 95, b: 128 }
console.log(primary.name)       // "Jandí"
console.log(primary.description) // "Oxidação completa..."

// Iterate the palette
palette.colors.forEach(c => console.log(c.name, c.hex))

// Look up by slug
const color = palette.bySlug['jandi-genipina']

// Color conversion utilities
hexToRgb('#3D5F80')   // { r: 61, g: 95, b: 128 }
hexToHsl('#3D5F80')   // { h: 210, s: 35, l: 37 }
isLight(primary)       // false (luminosity < 50%)

// WCAG contrast pairs (ratio >= 4.5:1)
for (const [fg, bg] of getContrastPair(palette)) {
  console.log(`${fg.name} on ${bg.name}`)
}
```

## Accessibility

WCAG 2.1 contrast combinations meeting AA (ratio >= 4.5:1):

| Background | Foreground | Ratio | Level |
|------------|-----------|-------|-------|
| Suco verde | Yandí | 10.7:1 | AAA |
| Suco verde | Nhandí | 8.3:1 | AAA |
| Suco verde | Genipina | 6.0:1 | AA |
| Brisa | Tinta de guerra | 7.3:1 | AAA |
| Tinta de guerra | Suco verde | 12.2:1 | AAA |
| Tinta de guerra | Brisa | 7.3:1 | AAA |
| Brisa | Yandí | 6.5:1 | AA |
| Jandí | Suco verde | 4.4:1 | AA Large |
| Tinta de guerra | Oby | 4.3:1 | AA Large |
| Oby | Suco verde | 2.8:1 | — |

## Running Tests

```bash
cd packages/typescript

# Install dependencies
npm install

# Run all tests (55 tests)
npm run test

# Watch mode
npm run test:watch

# Type check
npm run typecheck

# Lint
npm run lint

# Build
npm run build
```

## Package Structure

```
@jandi/colors/
├── dist/                    # Build output (ESM + CJS + .d.ts)
├── css/jandi.css            # CSS custom properties
├── scss/_jandi.scss         # SCSS variables + mixins
├── json/jandi.tokens.json   # W3C Design Tokens
├── tailwind/jandi.config.cjs # Tailwind plugin
├── src/                     # TypeScript source
└── tests/                   # Test suites
```

## Other Ecosystems

- **Rust:** [`jandi-colors`](../rust/README.md) — crate on crates.io
- **Swift, Kotlin:** raw token files in [`tokens/`](../../tokens/)

## License

[MIT](https://github.com/franciscotbjr/jandi-colors/blob/main/LICENSE)

---

<p align="center">
  <sub>A Brazilian palette. From fruit to screen.</sub>
</p>
