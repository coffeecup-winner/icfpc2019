use core::fmt::Display;

pub enum TerminalColor {
    Green,
    Yellow,
    Red,
    Blue,
    Magenta,
    Default,
}

impl TerminalColor {
    fn escape_code(&self) -> &'static str {
        match self {
            TerminalColor::Green => "\x1b[32m",
            TerminalColor::Yellow => "\x1b[33m",
            TerminalColor::Red => "\x1b[31m",
            TerminalColor::Blue => "\x1b[34m",
            TerminalColor::Magenta => "\x1b[35m",
            TerminalColor::Default => "\x1b[39m",
        }
    }
}

pub trait Colorizable {
    type Result;

    fn colorize(&self, color: TerminalColor) -> Self::Result;
}

impl<T> Colorizable for T
where T: Display {
    type Result = String;

    fn colorize(&self, color: TerminalColor) -> String {
        format!("{}{}{}", color.escape_code(), self, TerminalColor::Default.escape_code())
    }
}
