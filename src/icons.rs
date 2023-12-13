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
    PlaystationCircle,
    PlaystationSquare,
    PlaystationTriangle,
    PlaystationX,
    XboxA,
    XboxB,
    XboxX,
    XboxY,
}

impl Icon {
    pub fn as_char(&self) -> char {
        match self {
            Self::AlertSquareRounded => '\u{f810}',
            Self::InfoSquareRounded => '\u{f635}',
            Self::PlaystationCircle => '\u{f2ad}',
            Self::PlaystationSquare => '\u{f2ae}',
            Self::PlaystationTriangle => '\u{f2af}',
            Self::PlaystationX => '\u{f2b0}',
            Self::XboxA => '\u{f2b6}',
            Self::XboxB => '\u{f2b7}',
            Self::XboxX => '\u{f2b8}',
            Self::XboxY => '\u{f2b9}',
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

    pub fn text_small(self) -> Text<'static> {
        text(self.as_char().to_string())
            .font(ICONS)
            .size(36)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .line_height(1.0)
    }
}
