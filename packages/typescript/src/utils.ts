import type { JandiColor, JandiPalette, RGB, HSL } from './types'

export function hexToRgb(hex: string): RGB {
  const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
  if (!result) {
    const safe = typeof hex === 'string' ? hex.slice(0, 32).replace(/[^\w#]/g, '?') : '<non-string>'
    throw new Error(`Invalid hex color format: ${safe}`)
  }
  return {
    r: parseInt(result[1], 16),
    g: parseInt(result[2], 16),
    b: parseInt(result[3], 16),
  }
}

export function hexToHsl(hex: string): HSL {
  const rgb = hexToRgb(hex)
  const r = rgb.r / 255
  const g = rgb.g / 255
  const b = rgb.b / 255

  const max = Math.max(r, g, b)
  const min = Math.min(r, g, b)
  let h = 0
  let s = 0
  const l = (max + min) / 2

  if (max !== min) {
    const d = max - min
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min)
    switch (max) {
      case r:
        h = ((g - b) / d + (g < b ? 6 : 0)) / 6
        break
      case g:
        h = ((b - r) / d + 2) / 6
        break
      case b:
        h = ((r - g) / d + 4) / 6
        break
    }
  }

  return {
    h: Math.round(h * 360),
    s: Math.round(s * 100),
    l: Math.round(l * 100),
  }
}

export function isLight(color: JandiColor): boolean {
  return color.hsl.l > 50
}

function getLuminance(rgb: RGB): number {
  const [r, g, b] = [rgb.r, rgb.g, rgb.b].map(c => {
    c = c / 255
    return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4)
  })
  return 0.2126 * r + 0.7152 * g + 0.0722 * b
}

function getContrastRatio(rgb1: RGB, rgb2: RGB): number {
  const l1 = getLuminance(rgb1)
  const l2 = getLuminance(rgb2)
  const lighter = Math.max(l1, l2)
  const darker = Math.min(l1, l2)
  return (lighter + 0.05) / (darker + 0.05)
}

export function getContrastPair(palette: JandiPalette): [JandiColor, JandiColor][] {
  const validPairs: [JandiColor, JandiColor][] = []
  const colors = palette.colors

  for (let i = 0; i < colors.length; i++) {
    for (let j = i + 1; j < colors.length; j++) {
      const ratio = getContrastRatio(colors[i].rgb, colors[j].rgb)
      if (ratio >= 4.5) {
        validPairs.push([colors[i], colors[j]])
      }
    }
  }
  return validPairs
}

export function toCSS(color: JandiColor): string {
  return color.hex
}

export function toRGBA(color: JandiColor, alpha: number): string {
  if (typeof alpha !== 'number' || !Number.isFinite(alpha) || alpha < 0 || alpha > 1) {
    throw new Error('Invalid alpha value: must be a finite number between 0 and 1')
  }
  const { r, g, b } = color.rgb
  return `rgba(${r}, ${g}, ${b}, ${alpha})`
}
