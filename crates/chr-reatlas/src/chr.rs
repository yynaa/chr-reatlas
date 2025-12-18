use arbitrary_int::u2;
use bitvec::{order::Msb0, view::BitView};

pub type ChrPixelPattern = [[u2; 8]; 8];

// --- READING ---

/// read a single chr from bytes
pub fn read_single_chr(b: [u8; 16]) -> ChrPixelPattern {
  let mut r = [[u2::new(0); 8]; 8];

  let (plane_0, plane_1) = b.split_at(8);

  for (y, value) in plane_0.iter().enumerate() {
    for (x, on) in value.view_bits::<Msb0>().iter().enumerate() {
      if on == true {
        r[y][x] = r[y][x].saturating_add(u2::new(1));
      }
    }
  }

  for (y, value) in plane_1.iter().enumerate() {
    for (x, on) in value.view_bits::<Msb0>().iter().enumerate() {
      if on == true {
        r[y][x] = r[y][x].saturating_add(u2::new(2));
      }
    }
  }

  r
}

/// x flip a chr
pub fn flip_x(b: &mut ChrPixelPattern) {
  for bb in b {
    bb.reverse();
  }
}

/// y flip a chr
pub fn flip_y(b: &mut ChrPixelPattern) {
  b.reverse();
}

/// transpose a chr
pub fn transpose(b: &mut ChrPixelPattern) {
  for i in 1..8 {
    for j in 0..i {
      b[i][j] = b[j][i];
    }
  }
}

/// read a vector of chrs from bytes
pub fn read_bytes(b: Vec<u8>) -> Result<Vec<ChrPixelPattern>, crate::Error> {
  let slices = b.chunks(16);
  let mut chrs = Vec::new();

  for s in slices {
    chrs.push(read_single_chr(
      s.try_into().map_err(crate::Error::BytesError)?,
    ));
  }

  Ok(chrs)
}
