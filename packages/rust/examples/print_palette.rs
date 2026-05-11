use crossterm::style::{Color, Stylize};
use jandi_colors::PALETTE;

fn main() {
    println!("Jandí Color Palette\n");

    for color in PALETTE.iter() {
        let crossterm_color = Color::Rgb {
            r: color.rgb.r,
            g: color.rgb.g,
            b: color.rgb.b,
        };
        println!(
            "  {}  {}  (#{})  {:>3} {:>3} {:>3}",
            "    ".with(crossterm_color).on(crossterm_color),
            color.name,
            &color.hex[1..],
            color.rgb.r,
            color.rgb.g,
            color.rgb.b,
        );
    }

    println!("\nUse `cargo add jandi-colors` to use this palette in your project.");
}
