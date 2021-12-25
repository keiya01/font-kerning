use image::{Rgba, RgbaImage};
use fontdue::{FontSettings, Font as Fontdue};
use rusttype::{Font as Rusttype, Scale};
use ab_glyph::{FontRef};
use font_kerning::{draw_text_mut_fontdue, draw_text_mut_rusttype, draw_text_mut_ab, FontFeatureSettings, FontSetting};
use std::path::Path;

fn fontdue() {
  let mut canvas = RgbaImage::new(1200, 800);
  let color = Rgba([255u8, 0u8, 0u8, 255]);
  let font = include_bytes!("../fonts/NotoSansJP-Black.otf") as &[u8];
  let font = Fontdue::from_bytes(font, FontSettings::default()).unwrap();
  let text = "こんにちは、世界!!";
  draw_text_mut_fontdue(&mut canvas, color, 0, 0, 20., &font, text);
  canvas.save(Path::new("./images/fontdue.png")).unwrap();
}

fn ab() {
  let mut canvas = RgbaImage::new(1200, 800);
  let color = Rgba([255u8, 0u8, 0u8, 255]);
  let font = FontRef::try_from_slice(include_bytes!("../fonts/NotoSansJP-Black.otf")).unwrap();
  let text = "こんにちは、世界!!";
  draw_text_mut_ab(&mut canvas, color, 0, 0, &font, FontSetting { letter_spacing: 10, size: 100., font_feature_settings: FontFeatureSettings::Palt }, text);
  canvas.save(Path::new("./images/ab.png")).unwrap();
}

fn rusttype() {
  let mut canvas = RgbaImage::new(1200, 800);
  let color = Rgba([255u8, 0u8, 0u8, 255]);
  let font = include_bytes!("../fonts/NotoSansJP-Black.otf") as &[u8];
  let font = Rusttype::try_from_bytes(font).unwrap();
  let text = "こんにちは、世界!!";
  draw_text_mut_rusttype(&mut canvas, color, 0, 0, Scale::uniform(20.), &font, text);
  canvas.save(Path::new("./images/rusttype.png")).unwrap();
}

fn main() {
  rusttype();
  fontdue();
  ab();
}
