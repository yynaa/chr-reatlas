use anyhow::Result;
use chr_reatlas::{
  atlas::Atlas,
  chr::read_bytes,
  pal::{ChrPalette, read_palette_from_bytes},
  render::get_patterns_as_png_bytes,
};
use raylib::prelude::*;
use std::{
  fs::File,
  io::{Read, Seek, SeekFrom},
};

pub struct AtlasDisplay {
  pub binary_texture: Texture2D,
  pub atlas_texture: Option<Texture2D>,
  pub palette: Vec<[u8; 3]>,
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

    let atlas_texture = match a.data.len() {
      0 => None,
      _ => {
        let atlas_bytes = a.get_png_bytes()?;
        let atlas_image = Image::load_image_from_mem(".png", &atlas_bytes)?;
        Some(d.load_texture_from_image(thread, &atlas_image)?)
      }
    };

    let mut pal_file = File::open(&a.palette)?;
    let mut pal_buf = Vec::new();
    pal_file.read_to_end(&mut pal_buf)?;
    let palette = read_palette_from_bytes(pal_buf)?;

    Ok(Self {
      binary_texture,
      atlas_texture,
      palette,
    })
  }

  pub fn regen_atlas_texture(
    &mut self,
    d: &mut RaylibDrawHandle,
    thread: &RaylibThread,
    a: &Atlas,
  ) -> Result<()> {
    self.atlas_texture = match a.data.len() {
      0 => None,
      _ => {
        let atlas_bytes = a.get_png_bytes()?;
        let atlas_image = Image::load_image_from_mem(".png", &atlas_bytes)?;
        Some(d.load_texture_from_image(thread, &atlas_image)?)
      }
    };

    Ok(())
  }
}
