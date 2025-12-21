use raylib::prelude::*;

use crate::{Context, window::Window};

pub enum ColorPickerToChange {
  Color {
    currently_editing_data: usize,
    color_index: u32,
  },
  BackgroundColor {
    currently_editing_data: usize,
  },
}

pub struct ColorPicker {
  opened: bool,

  to_change: ColorPickerToChange,
}

pub enum ColorPickerMessage {}

impl ColorPicker {
  pub fn set_data(&mut self, to_change: ColorPickerToChange) {
    self.to_change = to_change;
  }
}

impl Window<Context, ColorPickerMessage> for ColorPicker {
  fn init() -> Self {
    Self {
      opened: false,
      to_change: ColorPickerToChange::BackgroundColor {
        currently_editing_data: 0,
      },
    }
  }

  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle {
    let width = d.get_screen_width();
    let height = d.get_screen_height();

    let window_width = 20. + 16. * 24.;
    let window_height = 20. + 32. * 16.;

    Rectangle::new(
      width as f32 / 2. - window_width / 2.,
      height as f32 / 2. - window_height / 2.,
      window_width,
      window_height,
    )
  }

  fn display(
    &mut self,
    d: &mut RaylibDrawHandle,
    t: &RaylibThread,
    c: &mut Context,
    i: bool,
  ) -> Vec<ColorPickerMessage> {
    let mut _r = Vec::new();

    let width = d.get_screen_width();
    let height = d.get_screen_height();

    d.draw_rectangle(0, 0, width, height, Color::new(0, 0, 0, 128));

    let r = self.get_rect(d);

    if d.gui_window_box(r, "color picker") {
      self.opened = false;
    }

    if let Some(ad) = &mut c.atlas_display {
      for (j, color) in ad.palette.clone().iter().enumerate() {
        let cr = Rectangle::new(
          r.x + (j % 16) as f32 * 24. + 10.,
          r.y + (j / 16) as f32 * 16. + 35.,
          24.,
          16.,
        );

        if d.gui_button(cr, "") && i {
          if let Some(a) = &mut c.atlas {
            match self.to_change {
              ColorPickerToChange::Color {
                currently_editing_data,
                color_index,
              } => {
                match color_index {
                  0 => a.data[currently_editing_data].c0 = j,
                  1 => a.data[currently_editing_data].c1 = j,
                  2 => a.data[currently_editing_data].c2 = j,
                  _ => panic!(),
                };
              }
              ColorPickerToChange::BackgroundColor {
                currently_editing_data,
              } => {
                a.data[currently_editing_data].cbg = Some(j);
              }
            }

            ad.regen_atlas_texture(d, t, a).unwrap();
            self.opened = false;
          }
        }

        d.draw_rectangle_rec(cr, Color::new(color[0], color[1], color[2], 255));
      }
    }

    _r
  }

  fn is_opened(&self) -> bool {
    self.opened
  }

  fn set_opened(&mut self, opened: bool) {
    self.opened = opened;
  }
}
