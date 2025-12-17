use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
};

use image::RgbaImage;

use crate::{
  chr::read_bytes,
  pal::read_palette,
  render::{ChrPalette, append_pattern_on_image},
};

#[derive(Clone, Debug)]
pub struct Atlas {
  pub bin: String,
  pub pal: String,
  pub start: u64,
  pub length: usize,
  pub data: Vec<AtlasData>,
}

#[derive(Clone, Debug)]
pub struct AtlasData {
  pub chr_index: usize,
  pub c0: usize,
  pub c1: usize,
  pub c2: usize,
  pub x: u32,
  pub y: u32,
}

macro_rules! error_mapper {
  ($s:expr) => {
    |e| crate::Error::AtlasParseError($s.to_string(), e)
  };
}

fn parse_line(c: &str) -> Result<AtlasData, crate::Error> {
  let s: Vec<&str> = c.split(",").collect();

  if s.len() != 6 {
    return Err(crate::Error::AtlasShapeError);
  }

  let chr_index = usize::from_str_radix(s[0], 16).map_err(error_mapper!(s[0]))?;
  let x = u32::from_str_radix(s[1], 8).map_err(error_mapper!(s[1]))?;
  let y = u32::from_str_radix(s[2], 8).map_err(error_mapper!(s[2]))?;
  let c0 = usize::from_str_radix(s[3], 16).map_err(error_mapper!(s[3]))?;
  let c1 = usize::from_str_radix(s[4], 16).map_err(error_mapper!(s[4]))?;
  let c2 = usize::from_str_radix(s[5], 16).map_err(error_mapper!(s[5]))?;

  Ok(AtlasData {
    chr_index,
    c0,
    c1,
    c2,
    x,
    y,
  })
}

pub fn parse_atlas(c: &str) -> Result<Atlas, crate::Error> {
  let s: Vec<&str> = c
    .lines()
    .filter(|x| x.len() > 0 && !x.starts_with("//"))
    .collect();

  if s.len() < 4 {
    return Err(crate::Error::AtlasShapeError);
  }

  let bin = s[0].to_string();
  let pal = s[1].to_string();
  let start = u64::from_str_radix(s[2], 16).map_err(error_mapper!(s[2]))?;
  let length = usize::from_str_radix(s[3], 16).map_err(error_mapper!(s[3]))?;
  let mut data = Vec::new();

  for i in 4..s.len() {
    data.push(parse_line(s[i])?);
  }

  Ok(Atlas {
    bin,
    pal,
    start,
    length,
    data,
  })
}

pub fn render_image_from_atlas(a: Atlas, output_path: String) -> Result<(), crate::Error> {
  let mut chr_file = File::open(a.bin).map_err(crate::Error::IOError)?;
  chr_file
    .seek(SeekFrom::Start(a.start * 16))
    .map_err(crate::Error::IOError)?;
  let mut chr_buf = vec![0; a.length * 16];
  chr_file
    .read_exact(&mut chr_buf)
    .map_err(crate::Error::IOError)?;

  let chrs = read_bytes(chr_buf.to_vec())?;

  let mut pal_file = File::open(a.pal).map_err(crate::Error::IOError)?;
  let mut pal_buf = Vec::new();
  pal_file
    .read_to_end(&mut pal_buf)
    .map_err(crate::Error::IOError)?;

  let pal = read_palette(pal_buf)?;

  let img_size = a
    .data
    .iter()
    .fold((0, 0), |acc, d| (acc.0.max(d.x + 8), acc.1.max(d.y + 8)));
  let mut img = RgbaImage::new(img_size.0, img_size.1);

  for d in a.data {
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

  img.save(output_path).map_err(crate::Error::ImageError)?;

  Ok(())
}
