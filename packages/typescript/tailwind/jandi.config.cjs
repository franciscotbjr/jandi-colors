/**
 * Jandí Colors — Tailwind CSS Plugin
 * Derived from the blue pigment of jenipapo (Genipa americana)
 *
 * Usage in tailwind.config.js:
 *   const { jandiColors } = require('@jandi/colors/tailwind')
 *   module.exports = { theme: { extend: { colors: jandiColors } } }
 *
 * Then use: bg-jandi-primary, text-jandi-oby, border-jandi-genipina, etc.
 */

const jandiColors = {
  jandi: {
    "suco-verde": "#C8D5CC",
    brisa: "#8FA7B3",
    oby: "#5D7F96",
    primary: "#3D5F80",
    genipina: "#2C4A6E",
    nhandi: "#1E3452",
    yandi: "#11203A",
    "tinta-guerra": "#0A1424",
  },
}

const jandiSemantic = {
  bg: {
    lightest: "var(--jandi-suco-verde)",
    light: "var(--jandi-brisa)",
    surface: "var(--jandi-nhandi)",
    base: "var(--jandi-yandi)",
    deep: "var(--jandi-tinta-guerra)",
  },
  text: {
    primary: "var(--jandi-suco-verde)",
    secondary: "var(--jandi-brisa)",
    muted: "var(--jandi-oby)",
  },
  accent: {
    default: "var(--jandi-primary)",
    strong: "var(--jandi-genipina)",
  },
}

module.exports = {
  jandiColors,
  jandiSemantic,
}