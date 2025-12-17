use raylib::{RaylibThread, math::Rectangle, prelude::RaylibDrawHandle};

pub(crate) mod atlas_creator;

#[macro_export]
macro_rules! make_text_input {
  ($d:expr, $buffer:expr, $edit:expr, $index:expr, $rect:expr, $interactable:expr) => {
    let mut buffer = $buffer[$index].clone();
    if $edit[$index] {
      let o = buffer;
      buffer = String::from("> ");
      buffer.push_str(&o);
    }
    if $d.gui_button($rect, &buffer) && $interactable {
      if $edit[$index] {
        $edit.fill(false);
      } else {
        $edit.fill(false);
        $edit[$index] = true;
      }
    }
  };
}

pub(crate) trait Window<C, M> {
  fn init() -> Self;
  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle;
  fn display(
    &mut self,
    _d: &mut RaylibDrawHandle,
    _t: &RaylibThread,
    _c: &mut C,
    _i: bool,
  ) -> Vec<M>;
  fn is_opened(&self) -> bool;
  fn set_opened(&mut self, opened: bool);
}
