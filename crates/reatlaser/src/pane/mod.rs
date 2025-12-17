use raylib::prelude::*;

pub mod file;
pub mod picker;

pub(crate) trait Pane<C, M> {
  fn init() -> Self;
  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle;
  fn display(
    &mut self,
    _d: &mut RaylibDrawHandle,
    _t: &RaylibThread,
    _c: &mut C,
    _i: bool,
  ) -> Vec<M>;
}
