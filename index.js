/**
 * Jandí Colors
 * Derived from the blue pigment of jenipapo (Genipa americana)
 */

const colors = {
  sucoVerde:   '#C8D5CC',
  brisa:       '#8FA7B3',
  oby:         '#5D7F96',
  primary:     '#3D5F80',
  genipina:    '#2C4A6E',
  nhandi:      '#1E3452',
  yandi:       '#11203A',
  tintaGuerra: '#0A1424',
}

const rgb = {
  sucoVerde:   [200, 213, 204],
  brisa:       [143, 167, 179],
  oby:         [93,  127, 150],
  primary:     [61,  95,  128],
  genipina:    [44,  74,  110],
  nhandi:      [30,  52,  82],
  yandi:       [17,  32,  58],
  tintaGuerra: [10,  20,  36],
}

const hsl = {
  sucoVerde:   [138, 12, 81],
  brisa:       [200, 17, 63],
  oby:         [204, 23, 48],
  primary:     [210, 35, 37],
  genipina:    [213, 43, 30],
  nhandi:      [215, 46, 22],
  yandi:       [218, 55, 15],
  tintaGuerra: [217, 57, 9],
}

// Named exports for tree-shaking
const sucoVerde   = colors.sucoVerde
const brisa       = colors.brisa
const oby         = colors.oby
const jandi       = colors.primary
const genipina    = colors.genipina
const nhandi      = colors.nhandi
const yandi       = colors.yandi
const tintaGuerra = colors.tintaGuerra

module.exports = {
  colors,
  rgb,
  hsl,
  sucoVerde,
  brisa,
  oby,
  jandi,
  genipina,
  nhandi,
  yandi,
  tintaGuerra,
}
