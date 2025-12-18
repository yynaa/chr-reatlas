use raylib::prelude::*;

use crate::{Context, SelectionType, pane::Pane};

pub struct SelectedPanel {}

pub enum SelectedPanelMessage {
  OpenColorPicker(usize, u32),
}

impl Pane<Context, SelectedPanelMessage> for SelectedPanel {
  fn init() -> Self {
    Self {}
  }

  fn is_opened(&self, c: &Context) -> bool {
    c.selected_data.is_some()
  }

  fn get_rect(&self, d: &raylib::prelude::RaylibDrawHandle) -> Rectangle {
    let width = d.get_screen_width();
    let height = d.get_screen_height();

    let window_width = 400.;
    let window_height = 200.;

    Rectangle::new(
      width as f32 - window_width - 10.,
      height as f32 - window_height - 10.,
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
  ) -> Vec<SelectedPanelMessage> {
    let mut _r = Vec::new();

    const WINDOW_DECORATION_SIZE: f32 = 25.;
    let mut inside_rect = self.get_rect(d);
    inside_rect.y += WINDOW_DECORATION_SIZE;
    inside_rect.height -= WINDOW_DECORATION_SIZE;

    if let Some(a) = &mut c.atlas {
      if let Some(ad) = &mut c.atlas_display {
        if let Some(st) = &c.selected_data {
          match st {
            SelectionType::Single(sd) => {
              d.gui_panel(self.get_rect(d), "selected data");

              d.draw_texture_pro(
                &ad.binary_texture,
                Rectangle::new(
                  (a.data[*sd].chr_index % 16) as f32 * 8.,
                  (a.data[*sd].chr_index / 16) as f32 * 8.,
                  8.,
                  8.,
                ),
                Rectangle::new(
                  inside_rect.x + inside_rect.width - 10. - 40.,
                  inside_rect.y + 10.,
                  40.,
                  40.,
                ),
                Vector2::zero(),
                0.,
                Color::WHITE,
              );

              d.gui_label(
                Rectangle::new(
                  inside_rect.x + inside_rect.width - 10. - 40.,
                  inside_rect.y + 50.,
                  40.,
                  10.,
                ),
                &format!("{}", a.data[*sd].chr_index),
              );

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 60.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                "Set C0",
              ) {
                _r.push(SelectedPanelMessage::OpenColorPicker(*sd, 0));
              }
              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 80.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                "Set C1",
              ) {
                _r.push(SelectedPanelMessage::OpenColorPicker(*sd, 1));
              }
              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 100.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                "Set C2",
              ) {
                _r.push(SelectedPanelMessage::OpenColorPicker(*sd, 2));
              }

              let c0 = ad.palette[a.data[*sd].c0];
              let c1 = ad.palette[a.data[*sd].c1];
              let c2 = ad.palette[a.data[*sd].c2];
              d.draw_rectangle_rec(
                Rectangle::new(
                  inside_rect.x + 10. + (inside_rect.width - 20.) / 3.,
                  inside_rect.y + 10. + 60.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                Color::new(c0[0], c0[1], c0[2], 255),
              );
              d.draw_rectangle_rec(
                Rectangle::new(
                  inside_rect.x + 10. + (inside_rect.width - 20.) / 3.,
                  inside_rect.y + 10. + 80.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                Color::new(c1[0], c1[1], c1[2], 255),
              );
              d.draw_rectangle_rec(
                Rectangle::new(
                  inside_rect.x + 10. + (inside_rect.width - 20.) / 3.,
                  inside_rect.y + 10. + 100.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                Color::new(c2[0], c2[1], c2[2], 255),
              );

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10. + 2. * (inside_rect.width - 20.) / 3.,
                  inside_rect.y + 10. + 60.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                "Tile |> Buffer",
              ) {
                c.default_colors = [a.data[*sd].c0, a.data[*sd].c1, a.data[*sd].c2]
              }

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10. + 2. * (inside_rect.width - 20.) / 3.,
                  inside_rect.y + 10. + 80.,
                  (inside_rect.width - 20.) / 3.,
                  20.,
                ),
                "Buffer |> Tile",
              ) {
                a.data[*sd].c0 = c.default_colors[0];
                a.data[*sd].c1 = c.default_colors[1];
                a.data[*sd].c2 = c.default_colors[2];
                ad.regen_atlas_texture(d, &t, a).unwrap();
              }

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 30.,
                  inside_rect.height - 20. - 40.,
                  20.,
                ),
                "Duplicate",
              ) {
                let mut dup = a.data[*sd].clone();
                dup.x += 2;
                dup.y += 2;
                a.data.push(dup);
                ad.regen_atlas_texture(d, t, a).unwrap();
              }

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10.,
                  inside_rect.height - 20. - 40.,
                  20.,
                ),
                "Delete",
              ) {
                a.data.remove(*sd);
                ad.regen_atlas_texture(d, t, a).unwrap();
                c.selected_data = None;
              }
            }
            SelectionType::Multiple(sds) => {
              d.gui_panel(self.get_rect(d), "selected data (multiple)");

              d.gui_label(
                Rectangle::new(
                  inside_rect.x + 10. + inside_rect.width / 2.,
                  inside_rect.y + 10.,
                  (inside_rect.width - 10.) / 2.,
                  10.,
                ),
                &format!("selected: {}", sds.len()),
              );

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 60.,
                  inside_rect.height - 20. - 40.,
                  20.,
                ),
                "Buffer |> Tile",
              ) {
                for sd in sds {
                  a.data[*sd].c0 = c.default_colors[0];
                  a.data[*sd].c1 = c.default_colors[1];
                  a.data[*sd].c2 = c.default_colors[2];
                }
                ad.regen_atlas_texture(d, t, a).unwrap();
              }

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10. + 30.,
                  inside_rect.height - 20. - 40.,
                  20.,
                ),
                "Duplicate",
              ) {
                let mut vmax = Vector2::zero();
                let mut vmin = Vector2::one() * 10000000.;
                for sd in sds {
                  vmin.x = vmin.x.min(a.data[*sd].x as f32);
                  vmin.y = vmin.y.min(a.data[*sd].y as f32);
                  vmax.x = vmax.x.max(a.data[*sd].x as f32);
                  vmax.y = vmax.y.max(a.data[*sd].y as f32);
                }
                let v = vmax - vmin;
                for sd in sds {
                  let mut dup = a.data[*sd].clone();
                  dup.x += v.x.ceil() as u32 + 8;
                  dup.y += v.y.ceil() as u32 + 8;
                  a.data.push(dup);
                }
                ad.regen_atlas_texture(d, t, a).unwrap();
              }

              if d.gui_button(
                Rectangle::new(
                  inside_rect.x + 10.,
                  inside_rect.y + 10.,
                  inside_rect.height - 20. - 40.,
                  20.,
                ),
                "Delete",
              ) {
                let mut sds_sort = sds.clone();
                sds_sort.sort();
                for sd in sds_sort.iter().rev() {
                  a.data.remove(*sd);
                }
                ad.regen_atlas_texture(d, t, a).unwrap();
                c.selected_data = None;
              }
            }
          }
        }
      }
    }

    _r
  }
}
