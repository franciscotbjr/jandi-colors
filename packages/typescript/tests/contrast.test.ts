import { describe, it, expect } from "vitest"
import { palette } from "../src/palette"

const wcagCombos = [
  { fg: "jandi-suco-verde", bg: "jandi-yandi", level: "AAA" },
  { fg: "jandi-suco-verde", bg: "jandi-nhandi", level: "AAA" },
  { fg: "jandi-suco-verde", bg: "jandi-genipina", level: "AA" },
  { fg: "jandi-brisa", bg: "jandi-tinta-guerra", level: "AAA" },
  { fg: "jandi-brisa", bg: "jandi-yandi", level: "AA" },
  { fg: "jandi-suco-verde", bg: "jandi-tinta-guerra", level: "AAA" },
  { fg: "jandi-brisa", bg: "jandi-tinta-guerra", level: "AAA" },
]

function getLuminance(rgb: { r: number; g: number; b: number }): number {
  const [r, g, b] = [rgb.r, rgb.g, rgb.b].map((c) => {
    c = c / 255
    return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4)
  })
  return 0.2126 * r + 0.7152 * g + 0.0722 * b
}

function getContrastRatio(fg: { r: number; g: number; b: number }, bg: { r: number; g: number; b: number }): number {
  const l1 = getLuminance(fg)
  const l2 = getLuminance(bg)
  const lighter = Math.max(l1, l2)
  const darker = Math.min(l1, l2)
  return (lighter + 0.05) / (darker + 0.05)
}

describe("WCAG contrast combinations from README", () => {
  for (const { fg, bg, level } of wcagCombos) {
    const fgColor = palette.bySlug[fg]
    const bgColor = palette.bySlug[bg]

    it(`${fg} on ${bg} meets ${level}`, () => {
      if (!fgColor || !bgColor) {
        throw new Error(`Color not found: ${fg} or ${bg}`)
      }

      const ratio = getContrastRatio(fgColor.rgb, bgColor.rgb)
      const minRatio = level === "AAA" ? 7 : 4.5

      expect(ratio).toBeGreaterThanOrEqual(minRatio)
    })
  }

  it("oby on suco-verde does not meet AA (2.8:1)", () => {
    const fg = palette.bySlug["jandi-oby"]
    const bg = palette.bySlug["jandi-suco-verde"]
    const ratio = getContrastRatio(fg.rgb, bg.rgb)
    expect(ratio).toBeLessThan(4.5)
  })

  it("suco-verde on primary does not meet AA (4.4:1)", () => {
    const fg = palette.bySlug["jandi-suco-verde"]
    const bg = palette.bySlug["jandi-primary"]
    const ratio = getContrastRatio(fg.rgb, bg.rgb)
    expect(ratio).toBeLessThan(4.5)
  })

  it("oby on tinta-guerra does not meet AA (4.3:1)", () => {
    const fg = palette.bySlug["jandi-oby"]
    const bg = palette.bySlug["jandi-tinta-guerra"]
    const ratio = getContrastRatio(fg.rgb, bg.rgb)
    expect(ratio).toBeLessThan(4.5)
  })
})