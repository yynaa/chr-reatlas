use std::num::ParseIntError;

use image::{Rgba, RgbaImage};

use crate::{chr::ChrPixelPattern, chr::read_bytes};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChrPalette {
  pub c0: [u8; 3],
  pub c1: [u8; 3],
  pub c2: [u8; 3],
}

impl Default for ChrPalette {
  fn default() -> Self {
    Self {
      c0: [86, 86, 86],
      c1: [170, 170, 170],
      c2: [255, 255, 255],
    }
  }
}

pub fn append_pattern_on_image(
  image: &mut RgbaImage,
  pat: ChrPixelPattern,
  sx: u32,
  sy: u32,
  pal: ChrPalette,
) {
  for x in 0..8usize {
    for y in 0..8usize {
      match pat[y][x].value() {
        0 => {}
        e => {
          image.put_pixel(
            x as u32 + sx,
            y as u32 + sy,
            match e {
              1 => Rgba([pal.c0[0], pal.c0[1], pal.c0[2], 255]),
              2 => Rgba([pal.c1[0], pal.c1[1], pal.c1[2], 255]),
              3 => Rgba([pal.c2[0], pal.c2[1], pal.c2[2], 255]),
              _ => panic!(),
            },
          );
        }
      }
    }
  }
}

pub fn render_pattern(
  path: String,
  pat: ChrPixelPattern,
  pal: ChrPalette,
) -> Result<(), crate::Error> {
  let mut img = RgbaImage::new(8, 8);

  append_pattern_on_image(&mut img, pat, 0, 0, pal);

  img.save(path).map_err(crate::Error::ImageError)?;

  Ok(())
}

pub fn render_patterns(
  path: String,
  pats: Vec<ChrPixelPattern>,
  pals: Vec<ChrPalette>,
) -> Result<(), crate::Error> {
  const PATS_PER_LINE: u32 = 16;
  let img_width = PATS_PER_LINE * 8;
  let img_height = (pats.len() as u32).div_ceil(PATS_PER_LINE) * 8;

  let mut img = RgbaImage::new(img_width, img_height);

  for (i, pat) in pats.iter().enumerate() {
    let y = (i as u32).div_euclid(PATS_PER_LINE);
    let x = (i as u32).rem_euclid(PATS_PER_LINE);
    append_pattern_on_image(&mut img, *pat, x * 8, y * 8, pals[i]);
  }

  img.save(path).map_err(crate::Error::ImageError)?;

  Ok(())
}

pub fn render_patterns_with_graduations(
  path: String,
  pats: Vec<ChrPixelPattern>,
  pals: Vec<ChrPalette>,
) -> Result<(), crate::Error> {
  const PATS_PER_LINE: u32 = 16;
  const LEFT_TILES_WIDTH: u32 = 4;

  const LETTERS: &[u8] = include_bytes!("text.chr");
  const LETTERS_PALETTE: ChrPalette = ChrPalette {
    c0: [255, 255, 255],
    c1: [255, 255, 255],
    c2: [255, 255, 255],
  };

  let text_chrs = read_bytes(LETTERS.to_vec())?;

  let img_width = (PATS_PER_LINE + LEFT_TILES_WIDTH) * 8;
  let img_height = (pats.len() as u32).div_ceil(PATS_PER_LINE) * 8 + 8;

  let mut img = RgbaImage::new(img_width, img_height);

  for (x, t) in text_chrs.iter().enumerate() {
    append_pattern_on_image(
      &mut img,
      *t,
      (x as u32 + LEFT_TILES_WIDTH) * 8,
      0,
      LETTERS_PALETTE,
    );
  }

  for (i, pat) in pats.iter().enumerate() {
    let y = (i as u32).div_euclid(PATS_PER_LINE);
    let x = (i as u32).rem_euclid(PATS_PER_LINE);

    if x == 0 {
      let formatted: Result<Vec<usize>, ParseIntError> = format!("{:X}", y)
        .char_indices()
        .try_fold(Vec::new(), |mut acc: Vec<usize>, (_, c)| {
          acc.push(usize::from_str_radix(&c.to_string(), 16)?);
          Ok(acc)
        });

      for (x, i) in formatted.unwrap().iter().rev().enumerate() {
        append_pattern_on_image(
          &mut img,
          text_chrs[*i],
          (LEFT_TILES_WIDTH - 1 - x as u32) * 8,
          (y + 1) * 8,
          LETTERS_PALETTE,
        );
      }
    }

    append_pattern_on_image(
      &mut img,
      *pat,
      (x + LEFT_TILES_WIDTH) * 8,
      (y + 1) * 8,
      pals[i],
    );
  }

  img.save(path).map_err(crate::Error::ImageError)?;

  Ok(())
}
