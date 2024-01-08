use fontdue::*;
use fontdue::layout::{CoordinateSystem, Layout, TextStyle, LayoutSettings};
use rayon::prelude::*;

use crate::{Pixel, Color};
struct TextRenderer {
    fonts: [Font; 1]
}

impl TextRenderer {
    fn new() -> TextRenderer {
        let font = include_bytes!("../resources/MapleMono-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        let fonts = [font];
        TextRenderer {
            fonts,
        }
    }

    fn render(&mut self, text: &str, scale: f32, color: Color) -> Vec<Pixel> {
        let mut pixels = Vec::new();
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        
        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });
        
        layout.append(&self.fonts, &TextStyle::new(text, scale, 0));
    
        let glyphs = layout.glyphs();

        pixels
    }
}