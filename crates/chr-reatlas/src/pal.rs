pub(crate) fn read_palette(b: Vec<u8>) -> Result<Vec<[u8; 3]>, crate::Error> {
  let slices = b.chunks(3);
  let mut pal = Vec::new();

  for s in slices {
    pal.push(s.try_into().map_err(crate::Error::BytesError)?);
  }

  Ok(pal)
}
