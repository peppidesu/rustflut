use rusttype::{point, Font, Scale};

use crate::{Pixel, Color, Point, PixelBuffer, Bounds};
pub struct TextRenderer {
    font: Font<'static>,
    scale: Scale,
    v_metrics: rusttype::VMetrics
}

impl TextRenderer {
    pub fn new() -> TextRenderer {
        let font_data = include_bytes!("../resources/Comfortaa-Regular.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
        let scale = Scale::uniform(48.0);
        let v_metrics = font.v_metrics(scale);
        TextRenderer {
            font,
            scale,
            v_metrics
        }
    }

    pub fn render_transparent(&self, text: &str, color: Color, pos: Point) -> Vec<Pixel> {
        
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
                    let px = Pixel::new(Point::new(x as u16, y as u16), c);
                    px_vec.push(px);
                });
            }
        }
        px_vec
    }
    pub fn render_bg(&self, text: &str, color: Color, pos: Point, buffer: &mut PixelBuffer) {
        
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

        let bounds = Bounds::new(
            Point::new(xmin as u16 + pos.x, ymin as u16 + pos.y), 
            Point::new((xmax+4) as u16 + pos.x, (ymax+4) as u16 + pos.y)
        );
        
        buffer.fill_bounds(
            bounds,
            Color::rgb(0, 0, 0)
        );
        
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                // glyph.draw(|x, y, v| {
                //     if v > 0.01 {
                //         let x = x as i32 + bb.min.x + 2;
                //         let y = y as i32 + bb.min.y + 2;
                //         let c = Color::rgba(
                //             color.r, color.g, color.b, (v * 255.0) as u8
                //         );
                //         buffer.set(Pixel::new(Point::new(x as u16 + pos.x, y as u16 + pos.y), c));
                //     }
                // });
                glyph.draw(|x, y, v| {
                    if v > 0.01 {
                        let x = x as i32 + bb.min.x + 2;
                        let y = y as i32 + bb.min.y + 2;
                        let c = Color::rgba(
                            color.r, color.g, color.b, (v * 255.0) as u8
                        );
                        buffer.set(Pixel::new(Point::new(x as u16 + pos.x, y as u16 + pos.y), c));
                    }
                });
            }
        }
        
    }
}