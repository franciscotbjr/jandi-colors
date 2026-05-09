package dev.jandi.colors

import androidx.compose.ui.graphics.Color

/**
 * Jandí Colors — derived from the blue pigment of jenipapo (Genipa americana)
 * https://github.com/franciscotbjr/jandi-colors
 */
object JandiColors {

    /** #C8D5CC — Pre-oxidation, fresh pulp extract */
    val SucoVerde   = Color(0xFFC8D5CC)

    /** #8FA7B3 — Light oxidation, first atmospheric contact */
    val Brisa       = Color(0xFF8FA7B3)

    /** #5D7F96 — Tupi blue (oby = blue/green in Tupi) */
    val Oby         = Color(0xFF5D7F96)

    /** #3D5F80 — Primary color, the revealed dye */
    val Primary     = Color(0xFF3D5F80)

    /** #2C4A6E — Deep indigo, full genipin oxidation */
    val Genipina    = Color(0xFF2C4A6E)

    /** #1E3452 — Night blue, multiple immersions */
    val Nhandi      = Color(0xFF1E3452)

    /** #11203A — Near-black, high concentration */
    val Yandi       = Color(0xFF11203A)

    /** #0A1424 — War paint, maximum ritual concentration */
    val TintaGuerra = Color(0xFF0A1424)

    /** All colors as an ordered list (lightest to darkest) */
    val all = listOf(SucoVerde, Brisa, Oby, Primary, Genipina, Nhandi, Yandi, TintaGuerra)
}
