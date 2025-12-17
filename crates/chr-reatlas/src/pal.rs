/// a palette for a chr
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ChrPalette {
  /// color 0
  pub c0: [u8; 3],
  /// color 1
  pub c1: [u8; 3],
  /// color 2
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

pub(crate) fn read_palette_from_bytes(b: Vec<u8>) -> Result<Vec<[u8; 3]>, crate::Error> {
  let slices = b.chunks(3);
  let mut pal = Vec::new();

  for s in slices {
    pal.push(s.try_into().map_err(crate::Error::BytesError)?);
  }

  Ok(pal)
}
