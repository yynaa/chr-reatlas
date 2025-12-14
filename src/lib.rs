use std::{array::TryFromSliceError, io, num::ParseIntError};

use image::ImageError;
use thiserror::Error;

pub mod atlas;
pub mod chr;
pub mod pal;
pub mod render;

#[derive(Debug, Error)]
pub enum Error {
  #[error("image error")]
  ImageError(ImageError),

  #[error("io error")]
  IOError(io::Error),

  #[error("bytes error")]
  BytesError(TryFromSliceError),

  #[error("atlas shape parse error")]
  AtlasShapeError,

  #[error("atlas chr index error")]
  AtlasWrongIndexError(usize),

  #[error("atlas parse error")]
  AtlasParseError(ParseIntError),
}
