use chr_reatlas::atlas::AtlasData;
use raylib::prelude::*;

use crate::{Context, pane::Pane};

pub struct PickerPanel {}

pub enum PickerPaneMessage {}

const SCALE: f32 = 3.;

impl Pane<Context, PickerPaneMessage> for PickerPanel {
  fn init() -> Self {
    Self {}
  }

  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle {
    let width = d.get_screen_width();
    let height = d.get_screen_height();

    let window_width = 8. * 16. * SCALE;
    let window_height = height as f32 - 20.;

    Rectangle::new(
      width as f32 - window_width - 10.,
      10.,
      window_width,
      window_height,
    )
  }

  fn display(
    &mut self,
    d: &mut RaylibDrawHandle,
    t: &RaylibThread,
    c: &mut Context,
    _i: bool,
  ) -> Vec<PickerPaneMessage> {
    let mut _r = Vec::new();

    const WINDOW_DECORATION_SIZE: f32 = 25.;
    let mut inside_rect = self.get_rect(d);
    inside_rect.y += WINDOW_DECORATION_SIZE;
    inside_rect.height -= WINDOW_DECORATION_SIZE;

    d.gui_panel(self.get_rect(d), "picker");

    if let Some(ad) = &mut c.atlas_display {
      if let Some(a) = &mut c.atlas {
        let n = a.length / 0x10;
        for j in 0..n {
          let x = j % 16;
          let y = j / 16;

          let rs = inside_rect.width / 16.;
          let rx = inside_rect.x + x as f32 * rs;
          let ry = inside_rect.y + y as f32 * rs;

          if d.gui_button(Rectangle::new(rx, ry, rs, rs), "") {
            a.data.push(AtlasData {
              chr_index: j,
              x: 0,
              y: 0,
              c0: 1,
              c1: 2,
              c2: 3,
            });
            ad.regen_atlas_texture(d, t, a).unwrap();
          }
        }
      }

      let texture_width = ad.binary_texture.width as f32;
      let texture_height = ad.binary_texture.height as f32;

      let mut source_rec = Rectangle::new(0., 0., texture_width, texture_height);

      let mut inside_rect = inside_rect.clone();

      if source_rec.height * SCALE < inside_rect.height {
        inside_rect.height = source_rec.height * SCALE;
      } else {
        source_rec.height = inside_rect.height / SCALE;
      }

      d.draw_texture_pro(
        &ad.binary_texture,
        source_rec,
        inside_rect,
        Vector2::zero(),
        0.,
        Color::WHITE,
      );
    }

    _r
  }
}
