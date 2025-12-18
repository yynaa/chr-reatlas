use image::{RgbaImage, codecs::png::PngEncoder};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
};

use crate::{
  chr::{flip_x, flip_y, read_bytes, transpose},
  pal::{ChrPalette, read_palette_from_bytes},
  render::append_pattern_on_image,
};

/// allows complex rendering of tiles or sprites from a chr binary
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Atlas {
  /// binary to source from
  pub binary: String,
  /// palette, must be a .pal file format
  pub palette: String,
  /// which address to start from
  pub start: u64,
  /// how many bytes to read
  pub length: usize,
  /// atlas data, contains every single tile to draw
  pub data: Vec<AtlasData>,
}

/// contains data for drawing one 8x8 tile
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct AtlasData {
  /// index of the tile
  pub chr_index: usize,
  /// color 0
  pub c0: usize,
  /// color 1
  pub c1: usize,
  /// color 2
  pub c2: usize,
  /// x position
  pub x: u32,
  /// y position
  pub y: u32,
  /// transpose
  #[serde(default)]
  pub transpose: bool,
  /// flip x
  #[serde(default)]
  pub flip_x: bool,
  /// flip y
  #[serde(default)]
  pub flip_y: bool,
}

impl Atlas {
  fn get_image(&self) -> Result<RgbaImage, crate::Error> {
    let mut chr_file = File::open(&self.binary).map_err(crate::Error::IOError)?;
    chr_file
      .seek(SeekFrom::Start(self.start))
      .map_err(crate::Error::IOError)?;
    let mut chr_buf = vec![0; self.length];
    chr_file
      .read_exact(&mut chr_buf)
      .map_err(crate::Error::IOError)?;

    let chrs = read_bytes(chr_buf.to_vec())?;

    let mut pal_file = File::open(&self.palette).map_err(crate::Error::IOError)?;
    let mut pal_buf = Vec::new();
    pal_file
      .read_to_end(&mut pal_buf)
      .map_err(crate::Error::IOError)?;

    let pal = read_palette_from_bytes(pal_buf)?;

    let img_size = self
      .data
      .iter()
      .fold((0, 0), |acc, d| (acc.0.max(d.x + 8), acc.1.max(d.y + 8)));
    let mut img = RgbaImage::new(img_size.0, img_size.1);

    for d in &self.data {
      if d.chr_index >= chrs.len() {
        return Err(crate::Error::AtlasWrongIndexError(d.chr_index));
      }

      let mut chr = chrs[d.chr_index];
      if d.transpose {
        transpose(&mut chr);
      }
      if d.flip_x {
        flip_x(&mut chr);
      }
      if d.flip_y {
        flip_y(&mut chr);
      }

      append_pattern_on_image(
        &mut img,
        chr,
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

  /// renders the atlas to a file
  pub fn render_image(&self, output_path: String) -> Result<(), crate::Error> {
    let img = self.get_image()?;
    img.save(output_path).map_err(crate::Error::ImageError)?;
    Ok(())
  }

  /// returns raw bytes for an image, allows crates to load them from memory
  pub fn get_png_bytes(&self) -> Result<Vec<u8>, crate::Error> {
    let img = self.get_image()?;

    let mut bytes = Vec::new();
    let encoder = PngEncoder::new(&mut bytes);
    img
      .write_with_encoder(encoder)
      .map_err(crate::Error::ImageError)?;

    Ok(bytes)
  }
}
