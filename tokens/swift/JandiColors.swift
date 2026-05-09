import SwiftUI

/// Jandí Colors — derived from the blue pigment of jenipapo (Genipa americana)
/// https://github.com/franciscotbjr/jandi-colors
public enum JandiColors {

    /// #C8D5CC — Pre-oxidation, fresh pulp extract
    public static let sucoVerde   = Color(red: 200/255, green: 213/255, blue: 204/255)

    /// #8FA7B3 — Light oxidation, first atmospheric contact
    public static let brisa       = Color(red: 143/255, green: 167/255, blue: 179/255)

    /// #5D7F96 — Tupi blue (oby = blue/green in Tupi)
    public static let oby         = Color(red: 93/255,  green: 127/255, blue: 150/255)

    /// #3D5F80 — Primary color, the revealed dye
    public static let primary     = Color(red: 61/255,  green: 95/255,  blue: 128/255)

    /// #2C4A6E — Deep indigo, full genipin oxidation
    public static let genipina    = Color(red: 44/255,  green: 74/255,  blue: 110/255)

    /// #1E3452 — Night blue, multiple immersions
    public static let nhandi      = Color(red: 30/255,  green: 52/255,  blue: 82/255)

    /// #11203A — Near-black, high concentration
    public static let yandi       = Color(red: 17/255,  green: 32/255,  blue: 58/255)

    /// #0A1424 — War paint, maximum ritual concentration
    public static let tintaGuerra = Color(red: 10/255,  green: 20/255,  blue: 36/255)
}

// MARK: - UIKit support
#if canImport(UIKit)
import UIKit

public extension UIColor {
    static let jandiSucoVerde   = UIColor(red: 200/255, green: 213/255, blue: 204/255, alpha: 1)
    static let jandiBrisa       = UIColor(red: 143/255, green: 167/255, blue: 179/255, alpha: 1)
    static let jandiOby         = UIColor(red: 93/255,  green: 127/255, blue: 150/255, alpha: 1)
    static let jandiPrimary     = UIColor(red: 61/255,  green: 95/255,  blue: 128/255, alpha: 1)
    static let jandiGenipina    = UIColor(red: 44/255,  green: 74/255,  blue: 110/255, alpha: 1)
    static let jandiNhandi      = UIColor(red: 30/255,  green: 52/255,  blue: 82/255,  alpha: 1)
    static let jandiYandi       = UIColor(red: 17/255,  green: 32/255,  blue: 58/255,  alpha: 1)
    static let jandiTintaGuerra = UIColor(red: 10/255,  green: 20/255,  blue: 36/255,  alpha: 1)
}
#endif
