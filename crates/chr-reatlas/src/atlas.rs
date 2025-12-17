use image::{EncodableLayout, RgbaImage};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
};

use crate::{
  chr::read_bytes,
  pal::read_palette,
  render::{ChrPalette, append_pattern_on_image},
};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Atlas {
  pub binary: String,
  pub palette: String,
  pub start: u64,
  pub length: usize,
  pub data: Vec<AtlasData>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AtlasData {
  pub chr_index: usize,
  pub c0: usize,
  pub c1: usize,
  pub c2: usize,
  pub x: u32,
  pub y: u32,
}

impl Atlas {
  fn get_image(&self) -> Result<RgbaImage, crate::Error> {
    let mut chr_file = File::open(&self.binary).map_err(crate::Error::IOError)?;
    chr_file
      .seek(SeekFrom::Start(self.start * 16))
      .map_err(crate::Error::IOError)?;
    let mut chr_buf = vec![0; self.length * 16];
    chr_file
      .read_exact(&mut chr_buf)
      .map_err(crate::Error::IOError)?;

    let chrs = read_bytes(chr_buf.to_vec())?;

    let mut pal_file = File::open(&self.palette).map_err(crate::Error::IOError)?;
    let mut pal_buf = Vec::new();
    pal_file
      .read_to_end(&mut pal_buf)
      .map_err(crate::Error::IOError)?;

    let pal = read_palette(pal_buf)?;

    let img_size = self
      .data
      .iter()
      .fold((0, 0), |acc, d| (acc.0.max(d.x + 8), acc.1.max(d.y + 8)));
    let mut img = RgbaImage::new(img_size.0, img_size.1);

    for d in &self.data {
      if d.chr_index >= chrs.len() {
        return Err(crate::Error::AtlasWrongIndexError(d.chr_index));
      }
      append_pattern_on_image(
        &mut img,
        chrs[d.chr_index],
        d.x,
        d.y,
        ChrPalette {
          c0: pal[d.c0],
          c1: pal[d.c1],
          c2: pal[d.c2],
        },
      );
    }

    Ok(img)
  }

  pub fn render_image(&self, output_path: String) -> Result<(), crate::Error> {
    let img = self.get_image()?;
    img.save(output_path).map_err(crate::Error::ImageError)?;
    Ok(())
  }

  pub fn get_image_bytes(&self) -> Result<Vec<u8>, crate::Error> {
    let img = self.get_image()?;
    Ok(img.as_bytes().to_vec())
  }
}
