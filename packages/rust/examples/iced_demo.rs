use iced_core::Color;
use jandi_colors::PALETTE;

fn main() {
    println!("Iced color swatches (rendered as terminal output since no GUI is attached):\n");
    for color in PALETTE.iter() {
        let iced_color: Color = (*color).into();
        println!("  {}  {}  {:?}", color.name, color.hex, iced_color,);
    }
    println!(
        "\nIn a real Iced application, use `Color::from(color)` to get `iced::Color` from a `JandiColor`."
    );
}
