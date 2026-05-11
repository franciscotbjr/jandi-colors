import { readFile } from 'fs/promises'
import { resolve } from 'path'
import { describe, it, expect } from 'vitest'
import {
  palette,
  sucoVerde,
  brisa,
  oby,
  primary,
  genipina,
  nhandi,
  yandi,
  tintaGuerra,
} from '../src/palette'
import { hexToRgb, hexToHsl } from '../src/utils'

const sourceOfTruth = JSON.parse(
  await readFile(resolve(__dirname, '../src/source-of-truth.json'), 'utf-8'),
)
const sourceColors = sourceOfTruth.colors

describe('source-of-truth has 8 colors', () => {
  it('should have exactly 8 colors', () => {
    expect(Object.keys(sourceColors).length).toBe(8)
  })
})

describe('palette.ts matches source-of-truth', () => {
  const colorEntries: [string, string][] = [
    ['jandi-suco-verde', 'suco-verde'],
    ['jandi-brisa', 'brisa'],
    ['jandi-oby', 'oby'],
    ['jandi-primary', 'primary'],
    ['jandi-genipina', 'genipina'],
    ['jandi-nhandi', 'nhandi'],
    ['jandi-yandi', 'yandi'],
    ['jandi-tinta-guerra', 'tinta-guerra'],
  ]

  for (const [slug, sourceKey] of colorEntries) {
    it(`${slug} hex matches source-of-truth`, () => {
      const color = palette.bySlug[slug]
      expect(color.hex).toBe(sourceColors[sourceKey])
    })
  }
})

describe('RGB values are mathematically correct', () => {
  const colors = [sucoVerde, brisa, oby, primary, genipina, nhandi, yandi, tintaGuerra]

  for (const color of colors) {
    it(`${color.slug} RGB matches hex conversion`, () => {
      expect(hexToRgb(color.hex)).toEqual(color.rgb)
    })
  }
})

describe('HSL values are mathematically correct', () => {
  const colors = [sucoVerde, brisa, oby, primary, genipina, nhandi, yandi, tintaGuerra]

  for (const color of colors) {
    it(`${color.slug} HSL matches hex conversion`, () => {
      expect(hexToHsl(color.hex)).toEqual(color.hsl)
    })
  }
})

describe('palette exports correct structure', () => {
  it('should have 8 colors in colors array', () => {
    expect(palette.colors.length).toBe(8)
  })

  it('should have bySlug with all colors', () => {
    expect(Object.keys(palette.bySlug).length).toBe(8)
  })

  it('version should be 0.1.0', () => {
    expect(palette.version).toBe('0.1.0')
  })
})
