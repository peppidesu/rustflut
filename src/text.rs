use rusttype::{point, Font, Scale};

use crate::{Pixel, Color, Pos};
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

    pub fn render_transparent(&self, text: &str, color: Color, pos: Pos) -> Vec<Pixel> {
        
        let glyphs: Vec<_> = self.font.layout(text, self.scale, point(0.0, 0.0 + self.v_metrics.ascent)).collect();
        let mut px_vec = Vec::new();
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v < 0.001 {
                        return;
                    }
                    let c = Color::rgba(
                        color.r, color.g, color.b, (v * 255.0) as u8
                    );
                    let x = x as i32 + bounding_box.min.x + pos.x as i32;
                    let y = y as i32 + bounding_box.min.y + pos.y as i32;
                    let px = Pixel::new(Pos::new(x as u16, y as u16), c);
                    px_vec.push(px);
                });
            }
        }
        px_vec
    }
    pub fn render_bg(&self, text: &str, color: Color, pos: Pos) -> Vec<Pixel> {
        
        let glyphs: Vec<_> = self.font.layout(text, self.scale, point(0.0, 0.0 + self.v_metrics.ascent)).collect();
        let mut xmin = 0;
        let mut ymin = 0;
        let mut xmax = 0;
        let mut ymax = 0;

        for glyph in &glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                if bb.min.x < xmin {
                    xmin = bb.min.x;
                }
                if bb.min.y < ymin {
                    ymin = bb.min.y;
                }
                if bb.max.x > xmax {
                    xmax = bb.max.x;
                }
                if bb.max.y > ymax {
                    ymax = bb.max.y;
                }
            }
        }
        
        let width = xmax - xmin + 2;
        let height = ymax - ymin + 2;

        let mut bitmap = Vec::new();
        bitmap.resize((width * height) as usize, 0u8);

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    if v > 0.001 {
                        let x = x as i32 + bb.min.x + 1;
                        let y = y as i32 + bb.min.y + 1;
                        let idx = (x + y * width) as usize;
                        bitmap[idx] = (v * 255.0) as u8;
                    }
                });
            }
        }
        
        let mut px_vec = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let idx = (x + y * width) as usize;
                let x = x as i32 + pos.x as i32 + xmin;
                let y = y as i32 + pos.y as i32 + ymin;
                if bitmap[idx] > 0 {
                    
                    let px = Pixel::new(Pos::new(x as u16, y as u16), color);
                    px_vec.push(px);
                }
                else {
                    let px = Pixel::new(Pos::new(x as u16, y as u16), Color::rgb(0, 0, 0));
                    px_vec.push(px);
                }
            }
        }
        px_vec
    }
}