use raylib::prelude::*;

pub mod file;
pub mod picker;
pub mod selected;

pub(crate) trait Pane<C, M> {
  fn init() -> Self;
  fn is_opened(&self, _c: &C) -> bool;
  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle;
  fn display(
    &mut self,
    _d: &mut RaylibDrawHandle,
    _t: &RaylibThread,
    _c: &mut C,
    _i: bool,
  ) -> Vec<M>;
}
