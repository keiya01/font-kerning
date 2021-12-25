use imageproc::definitions::{Clamp};
use imageproc::drawing::Canvas;
use conv::ValueInto;
use image::{Pixel};
use std::f32;
use std::i32;
use imageproc::pixelops::weighted_sum;
use fontdue::Font as Fontdue;
use rusttype::{Font as FontRusttype, Scale, PositionedGlyph, point};
use ab_glyph::{FontRef, Font, Glyph, point as ab_point, ScaleFont};

pub fn draw_text_mut_fontdue<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    scale: f32,
    font: &'a Fontdue,
    text: &'a str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let mut current_x = 0;
    for (_, ch) in text.char_indices() {
        let (metrics, pixels) = font.rasterize(ch, scale);
        for gy in 0..metrics.height {
            for gx in 0..metrics.width {
                let image_x = gx as i32 + metrics.xmin + current_x + x as i32;
                let bearing_y = metrics.advance_height as i32 - metrics.height as i32 - metrics.ymin;
                let image_y = gy as i32 + bearing_y + y as i32;

                let image_width = canvas.width() as i32;
                let image_height = canvas.height() as i32;

                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                    let v = pixels[gx as usize + gy as usize * metrics.width] as f32 / 255.;
                    let weighted_color = weighted_sum(pixel, color, 1.0 - v, v);
                    canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            }
        }
        current_x += metrics.width as i32;
    }
}

pub enum FontFeatureSettings {
    Normal,
    Palt,
}

pub struct FontSetting {
    pub letter_spacing: i32,
    pub size: f32,
    pub font_feature_settings: FontFeatureSettings,
}

impl Default for FontSetting {
    fn default() -> FontSetting {
        FontSetting {
            size: 16.,
            letter_spacing: 0,
            font_feature_settings: FontFeatureSettings::Normal,
        }
    }
}

pub fn draw_text_mut_ab<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    font: &'a FontRef,
    setting: FontSetting,
    text: &'a str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let mut current_x = 0;
    let scaled_font = font.as_scaled(setting.size);
    for (_, ch) in text.char_indices() {
        let glyph_id = font.glyph_id(ch);
        let v_bearing = scaled_font.v_side_bearing(glyph_id);
        let q_glyph: Glyph = glyph_id.with_scale_and_position(setting.size, ab_point(x as f32, y as f32));
        if let Some(q) = font.outline_glyph(q_glyph) {
            let bb = q.px_bounds();
            q.draw(|gx, gy, gv| {
                let gx = gx as i32 + current_x;
                let gy = gy as i32 + v_bearing as i32;

                let image_x = gx + x as i32;
                let image_y = gy + y as i32;

                let image_width = canvas.width() as i32;
                let image_height = canvas.height() as i32;

                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            });

            current_x += match setting.font_feature_settings {
                FontFeatureSettings::Normal => {
                    scaled_font.h_advance(glyph_id) as i32
                },
                FontFeatureSettings::Palt => {
                    bb.width() as i32
                },
            } + setting.letter_spacing;
        }
    }
}

pub fn draw_text_mut_rusttype<'a, C>(
    canvas: &'a mut C,
    color: C::Pixel,
    x: u32,
    y: u32,
    scale: Scale,
    font: &'a FontRusttype<'a>,
    text: &'a str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: ValueInto<f32> + Clamp<f32>,
{
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<PositionedGlyph<'_>> = font.layout(text, scale, offset).collect();

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|gx, gy, gv| {
                let gx = gx as i32 + bb.min.x;
                let gy = gy as i32 + bb.min.y;

                let image_x = gx + x as i32;
                let image_y = gy + y as i32;

                let image_width = canvas.width() as i32;
                let image_height = canvas.height() as i32;

                if image_x >= 0 && image_x < image_width && image_y >= 0 && image_y < image_height {
                    let pixel = canvas.get_pixel(image_x as u32, image_y as u32);
                    let weighted_color = weighted_sum(pixel, color, 1.0 - gv, gv);
                    canvas.draw_pixel(image_x as u32, image_y as u32, weighted_color);
                }
            })
        }
    }
}
