use anyhow::Result;
use chr_reatlas::{
  atlas::Atlas, chr::read_bytes, pal::ChrPalette, render::get_patterns_as_png_bytes,
};
use raylib::prelude::*;
use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
};

pub struct AtlasDisplay {
  pub binary_texture: Texture2D,
}

impl AtlasDisplay {
  pub fn from_atlas(d: &mut RaylibDrawHandle, thread: &RaylibThread, a: &Atlas) -> Result<Self> {
    let mut file = File::open(a.binary.clone())?;
    file.seek(SeekFrom::Start(a.start))?;
    let mut buf = vec![0; a.length];
    file.read_exact(&mut buf)?;

    let chrs = read_bytes(buf)?;
    let pals = vec![ChrPalette::default(); chrs.len()];
    let binary_patterns_bytes = get_patterns_as_png_bytes(chrs, pals)?;
    let binary_image = Image::load_image_from_mem(".png", &binary_patterns_bytes)?;
    let binary_texture = d.load_texture_from_image(thread, &binary_image)?;

    Ok(Self { binary_texture })
  }
}
