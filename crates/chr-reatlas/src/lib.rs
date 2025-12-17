//! # chr-reatlas
//!
//! *a renderer/reatlaser for CHR tables in .nes roms*
//!
//! if you are looking to make a spritesheet out of chrs, you are looking for [`Atlas`](atlas::Atlas)es \
//! if you are looking to render helpers, you are looking for the [`render`] module
//!

use std::{array::TryFromSliceError, io, num::ParseIntError};

use image::ImageError;
use thiserror::Error;

pub mod atlas;
pub mod chr;
pub mod pal;
pub mod render;

#[derive(Debug, Error)]
pub enum Error {
  #[error("image error: {0}")]
  ImageError(ImageError),

  #[error("io error: {0}")]
  IOError(io::Error),

  #[error("bytes error: {0}")]
  BytesError(TryFromSliceError),

  #[error("atlas shape parse error")]
  AtlasShapeError,

  #[error("atlas chr index error: {0}")]
  AtlasWrongIndexError(usize),

  #[error("atlas parse error: {0} - {1}")]
  AtlasParseError(String, ParseIntError),
}
