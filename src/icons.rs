use iced::{
    alignment, font,
    widget::{text, Text},
    Font,
};

// Stripped down version of the tabler icons font with only the ones used in
// enum Icon
pub const ICONS_DATA: &[u8] = include_bytes!("../assets/tabler-icons.ttf");
pub const ICONS: Font = Font {
    family: font::Family::Name("tabler-icons"),
    weight: font::Weight::Normal,
    stretch: font::Stretch::Normal,
    monospaced: true,
};

pub enum Icon {
    AlertSquareRounded,
    InfoSquareRounded,
}

impl Icon {
    pub fn as_char(&self) -> char {
        match self {
            Self::AlertSquareRounded => '\u{f810}',
            Self::InfoSquareRounded => '\u{f635}',
        }
    }

    pub fn text(self) -> Text<'static> {
        text(self.as_char().to_string())
            .font(ICONS)
            .size(54)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .line_height(1.0)
    }
}
