import type { JandiColor, JandiPalette, OxidationStage } from './types'

export const sucoVerde: JandiColor = {
  name: 'Suco verde',
  slug: 'jandi-suco-verde',
  hex: '#C8D5CC',
  rgb: { r: 200, g: 213, b: 204 },
  hsl: { h: 138, s: 13, l: 81 },
  oxidationStage: 'pre' as OxidationStage,
  description:
    'Polpa recém-extraída do fruto verde. O geniposídeo foi liberado pela trituração, mas a β-glucosidase ainda não completou a hidrólise. Nenhuma reação com proteínas ocorreu.',
}

export const brisa: JandiColor = {
  name: 'Brisa',
  slug: 'jandi-brisa',
  hex: '#8FA7B3',
  rgb: { r: 143, g: 167, b: 179 },
  hsl: { h: 200, s: 19, l: 63 },
  oxidationStage: 'initial' as OxidationStage,
  description:
    'Primeiro contato com o oxigênio atmosférico. A genipina livre começa a reagir com aminoácidos, formando os primeiros intermediários amarelo-acinzentados. A cor é sutil — como a brisa que antecede a mudança.',
}

export const oby: JandiColor = {
  name: 'Oby',
  slug: 'jandi-oby',
  hex: '#5D7F96',
  rgb: { r: 93, g: 127, b: 150 },
  hsl: { h: 204, s: 23, l: 48 },
  oxidationStage: 'partial' as OxidationStage,
  description:
    'O azul emerge. Os cromóforos de genipina-aminoácido estão se formando ativamente — a reticulação proteica está em progresso mas incompleta. O tom é reconhecivelmente azul mas ainda translúcido.',
}

export const primary: JandiColor = {
  name: 'Jandí',
  slug: 'jandi-primary',
  hex: '#3D5F80',
  rgb: { r: 61, g: 95, b: 128 },
  hsl: { h: 210, s: 35, l: 37 },
  oxidationStage: 'full' as OxidationStage,
  description:
    'Oxidação completa em concentração padrão. Todos os sítios de ligação genipina-proteína foram ocupados. Este é o azul que os povos indígenas obtêm na primeira aplicação sobre a pele — a tinta revelada.',
}

export const genipina: JandiColor = {
  name: 'Genipina',
  slug: 'jandi-genipina',
  hex: '#2C4A6E',
  rgb: { r: 44, g: 74, b: 110 },
  hsl: { h: 213, s: 43, l: 30 },
  oxidationStage: 'saturated' as OxidationStage,
  description:
    'Extrato concentrado com excesso de genipina livre. A proporção genipina/proteína é maior que a estequiométrica — camadas sobrepostas de pigmento produzem índigo natural profundo.',
}

export const nhandi: JandiColor = {
  name: 'Nhandí',
  slug: 'jandi-nhandi',
  hex: '#1E3452',
  rgb: { r: 30, g: 52, b: 82 },
  hsl: { h: 215, s: 46, l: 22 },
  oxidationStage: 'deep' as OxidationStage,
  description:
    'Múltiplas imersões. A pele é pintada, seca e pintada novamente — cada camada adiciona densidade ao pigmento reticulado na epiderme. O efeito cumulativo produz um azul-noite que absorve quase toda a luz visível.',
}

export const yandi: JandiColor = {
  name: 'Yandí',
  slug: 'jandi-yandi',
  hex: '#11203A',
  rgb: { r: 17, g: 32, b: 58 },
  hsl: { h: 218, s: 55, l: 15 },
  oxidationStage: 'concentrated' as OxidationStage,
  description:
    'Extrato reduzido por evaporação. O líquido foi fervido ou exposto ao sol para concentrar a genipina — técnica usada para preparar tinta de longa duração para cerâmica e tecidos.',
}

export const tintaGuerra: JandiColor = {
  name: 'Tinta de guerra',
  slug: 'jandi-tinta-guerra',
  hex: '#0A1424',
  rgb: { r: 10, g: 20, b: 36 },
  hsl: { h: 217, s: 57, l: 9 },
  oxidationStage: 'max' as OxidationStage,
  description:
    'Concentração máxima ritual. Extrato reduzido ao limite, aplicado em múltiplas camadas sobre pele escarificada para maximizar a penetração proteica. Indistinguível do preto a olho nu — só revela seu azul sob luz rasante.',
}

export const jandi = primary

export const palette: JandiPalette = {
  version: '0.1.0',
  colors: [sucoVerde, brisa, oby, primary, genipina, nhandi, yandi, tintaGuerra],
  bySlug: {
    'jandi-suco-verde': sucoVerde,
    'jandi-brisa': brisa,
    'jandi-oby': oby,
    'jandi-primary': primary,
    'jandi-genipina': genipina,
    'jandi-nhandi': nhandi,
    'jandi-yandi': yandi,
    'jandi-tinta-guerra': tintaGuerra,
  },
}
