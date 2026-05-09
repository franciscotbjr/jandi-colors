# @jandi/colors

**Uma paleta de cores derivada do pigmento azul do jenipapo (*Genipa americana*), a tinta ancestral dos povos indígenas do Brasil.**

<p align="center">
  <img src="https://raw.githubusercontent.com/franciscotbjr/jandi-colors/main/docs/assets/palette-banner.svg" alt="Jandí color palette" width="400">
</p>

---

## Instalação

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
      colors: jandiColors
    }
  }
}
```

```html
<div class="bg-jandi-primary text-jandi-brisa">...</div>
```

### SCSS

```scss
@import '@jandi/colors/scss/jandi';

.element {
  background: $jandi-primary;
  color: $jandi-text-primary;
}
```

## API TypeScript

```typescript
import { palette, primary, oby, sucoVerde } from '@jandi/colors'

console.log(primary.hex)          // "#3D5F80"
console.log(primary.rgb)        // { r: 61, g: 95, b: 128 }
console.log(primary.name)        // "Jandí"
console.log(primary.description) // "Oxidação completa..."

// Iterar sobre a paleta
palette.colors.forEach(c => console.log(c.name, c.hex))

// Buscar por slug
const color = palette.bySlug['jandi-genipina']
```

## Paleta Completa

| Nome | Hex | RGB | HSL | Estágio |
|------|-----|-----|-----|-------|
| **Suco verde** | #C8D5CC | 200, 213, 204 | 138°, 13%, 81% | pre |
| **Brisa** | #8FA7B3 | 143, 167, 179 | 200°, 19%, 63% | initial |
| **Oby** | #5D7F96 | 93, 127, 150 | 204°, 23%, 48% | partial |
| **Jandí** | #3D5F80 | 61, 95, 128 | 210°, 35%, 37% | full |
| **Genipina** | #2C4A6E | 44, 74, 110 | 213°, 43%, 30% | saturated |
| **Nhandí** | #1E3452 | 30, 52, 82 | 215°, 46%, 22% | deep |
| **Yandí** | #11203A | 17, 32, 58 | 218°, 55%, 15% | concentrated |
| **Tinta de guerra** | #0A1424 | 10, 20, 36 | 217°, 57%, 9% | max |

## Utilitários

```typescript
import { hexToRgb, hexToHsl, isLight, getContrastPair } from '@jandi/colors'

hexToRgb('#3D5F80')  // { r: 61, g: 95, b: 128 }
hexToHsl('#3D5F80') // { h: 210, s: 35, l: 37 }
isLight(primary)     // false (luminosity < 50%)

// Obter pares de contraste válidos (WCAG AA+)
getContrastPair(palette)
```

## Testes

```bash
cd packages/typescript

# Instalar dependencias
npm install

# Rodar todos os testes (38 testes)
npm run test

# Modo watch
npm run test:watch

# Type check
npm run typecheck

# Lint
npm run lint

# Build
npm run build
```

## Estrutura

```
@jandi/colors/
├── dist/              # Build output (ESM + CJS + .d.ts)
├── css/jandi.css      # CSS custom properties
├── scss/_jandi.scss  # SCSS variables + mixins
├── json/jandi.tokens.json  # W3C Design Tokens
└── tailwind/jandi.config.js # Tailwind plugin
```

## Acessibilidade

Combinações que atendem WCAG 2.1 AA (contraste ≥ 4.5:1):

| Fundo | Texto | Ratio |
|-------|-------|-------|
| Suco verde | Yandí | 10.2:1 |
| Suco verde | Nhandí | 7.1:1 |
| Brisa | Tinta de guerra | 11.8:1 |
| Tinta de guerra | Suco verde | 14.2:1 |

---

## Outros Ecossistemas

Para Rust, Swift, Kotlin: [consulte a raiz do repositório](https://github.com/franciscotbjr/jandi-colors).

## Licença

[MIT](https://github.com/franciscotbjr/jandi-colors/blob/main/LICENSE)

---

<p align="center">
  <sub>Uma paleta brasileira. Do fruto à tela.</sub>
</p>