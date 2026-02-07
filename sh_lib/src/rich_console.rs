pub enum TextColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Reset,
}

impl TextColor {
    fn as_code(&self) -> &str {
        match self {
            TextColor::Red => "31m",
            TextColor::Green => "32m",
            TextColor::Yellow => "33m",
            TextColor::Blue => "34m",
            TextColor::Magenta => "35m",
            TextColor::Cyan => "36m",
            TextColor::Reset => "0m",
        }
    }
}

pub fn colored_println(text: &str, color: TextColor) {
    println!(
        "\x1b[{}{}\x1b[{}",
        color.as_code(),
        text,
        TextColor::Reset.as_code()
    );
}
