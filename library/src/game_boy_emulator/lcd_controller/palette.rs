// Copyright 2023 Remi Bernotavicius
use super::LcdShade;
use crate::rendering::Color;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Palette {
    shade0: Color,
    shade1: Color,
    shade2: Color,
    shade3: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            shade0: Color::new(0xe0, 0xf8, 0xd0),
            shade1: Color::new(0x88, 0xc0, 0x70),
            shade2: Color::new(0x34, 0x68, 0x56),
            shade3: Color::new(0x08, 0x18, 0x20),
        }
    }
}

impl Palette {
    #[cfg_attr(feature = "aggressive-inline", inline(always))]
    pub(crate) fn color_for_shade(&self, shade: LcdShade) -> Color {
        match shade {
            LcdShade::Shade0 => self.shade0,
            LcdShade::Shade1 => self.shade1,
            LcdShade::Shade2 => self.shade2,
            LcdShade::Shade3 => self.shade3,
        }
    }
}
