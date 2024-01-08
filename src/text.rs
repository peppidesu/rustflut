use rusttype::{point, Font, Scale};

use crate::{Pixel, Color};
pub struct TextRenderer {
    font: Font<'static>,
    scale: Scale,
    v_metrics: rusttype::VMetrics
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        let font_data = include_bytes!("../resources/Comfortaa-Regular.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
        let scale = Scale::uniform(64.0);
        let v_metrics = font.v_metrics(scale);
        TextRenderer {
            font,
            scale,
            v_metrics
        }
    }

    pub fn render(&self, text: &str, color: Color) -> Vec<Pixel> {
        let glyphs: Vec<_> = self.font.layout(text, self.scale, point(0.0, 0.0 + self.v_metrics.ascent)).collect();
        let mut px_vec = Vec::new();
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v < 0.05 {
                        return;
                    }
                    let c = Color::rgba(
                        color.r, color.g, color.b, (v * 255.0) as u8
                    );
                    let x = x as i32 + bounding_box.min.x;
                    let y = y as i32 + bounding_box.min.y;
                    let px = Pixel::new(x as u16, y as u16, c);
                    px_vec.push(px);
                });
            }
        }
        px_vec
    }
}