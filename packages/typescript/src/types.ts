export type OxidationStage =
  | 'pre'
  | 'initial'
  | 'partial'
  | 'full'
  | 'saturated'
  | 'deep'
  | 'concentrated'
  | 'max'

export interface RGB {
  r: number
  g: number
  b: number
}

export interface HSL {
  h: number
  s: number
  l: number
}

export interface JandiColor {
  name: string
  slug: string
  description: string
  oxidationStage: OxidationStage
  hex: string
  rgb: RGB
  hsl: HSL
}

export interface JandiPalette {
  version: string
  colors: JandiColor[]
  bySlug: Record<string, JandiColor>
}
