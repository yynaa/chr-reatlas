use std::time::Instant;

use chr_reatlas::atlas::Atlas;
use raylib::prelude::*;

use crate::{Context, SelectionType};

enum LeftClickMode {
  NoModifiers,
  Control,
}

pub struct Editor {
  camera: Camera2D,

  lc_mode: LeftClickMode,
  lc_hold: bool,
  lc_pressed_time: Instant,
  lc_pressed_pos: Vector2,
  moving: bool,
  selecting_rectangle: bool,
}

pub enum EditorMessages {}

const TIME_BEFORE_HOLD: f32 = 0.2;

impl Editor {
  pub fn init() -> Self {
    Self {
      camera: Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.,
        zoom: 3.,
      },

      lc_mode: LeftClickMode::NoModifiers,
      lc_hold: false,
      lc_pressed_time: Instant::now(),
      lc_pressed_pos: Vector2::zero(),
      moving: false,
      selecting_rectangle: false,
    }
  }

  fn select_single_data_at_position(
    &mut self,
    a: &Atlas,
    projected_mouse: Vector2,
  ) -> Option<SelectionType> {
    let mut r = None;
    for (j, data) in a.data.iter().enumerate().rev() {
      if r.is_none()
        && Rectangle::new(data.x as f32, data.y as f32, 8., 8.)
          .check_collision_point_rec(projected_mouse)
      {
        r = Some(SelectionType::Single(j));
      }
    }
    r
  }

  pub fn display(
    &mut self,
    d: &mut RaylibDrawHandle,
    t: &RaylibThread,
    c: &mut Context,
    no_passthrough_rects: Vec<Rectangle>,
    i: bool,
  ) -> Vec<EditorMessages> {
    let mut _r = Vec::new();

    if let Some(a) = &mut c.atlas {
      if let Some(ad) = &mut c.atlas_display {
        let mut dc = d.begin_mode2D(self.camera);

        // --- INPUTS ---
        let mouse = dc.get_mouse_position();
        let projected_mouse = dc.get_screen_to_world2D(mouse, self.camera);
        let mut mouse_in_npr = false;
        for npr in no_passthrough_rects {
          if npr.check_collision_point_rec(mouse) {
            mouse_in_npr = true;
          }
        }
        if !mouse_in_npr && i {
          // can do actions in editor

          self.camera.zoom += dc.get_mouse_wheel_move() * 0.25;

          if dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            self.camera.target -= dc.get_mouse_delta() / self.camera.zoom;
          }

          if dc.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if dc.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
              self.lc_mode = LeftClickMode::Control;
            } else {
              self.lc_mode = LeftClickMode::NoModifiers;
            }
            self.lc_pressed_time = Instant::now();
            self.lc_pressed_pos = projected_mouse;
            self.lc_hold = false;
          }

          let now_hold = self.lc_pressed_time.elapsed().as_secs_f32() > TIME_BEFORE_HOLD;
          if dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && !self.lc_hold && now_hold {
            self.lc_hold = true;
            // HOLD SWITCH EVENT
            match self.lc_mode {
              LeftClickMode::NoModifiers => {
                match c.selected_data {
                  Some(SelectionType::Multiple(_)) => {}
                  _ => {
                    c.selected_data = self.select_single_data_at_position(a, self.lc_pressed_pos);
                  }
                }
                self.moving = true;
              }
              LeftClickMode::Control => {
                self.selecting_rectangle = true;
              }
            }
          }

          if dc.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            if self.lc_hold {
              // HOLDING
              match self.lc_mode {
                LeftClickMode::NoModifiers => {
                  self.moving = false;
                  if let Some(st) = &c.selected_data {
                    match st {
                      SelectionType::Single(sd) => {
                        a.data[*sd].x = (projected_mouse.x - 4.).round() as u32;
                        a.data[*sd].y = (projected_mouse.y - 4.).round() as u32;
                        ad.regen_atlas_texture(&mut dc, t, a).unwrap();
                        c.selected_data = None;
                      }
                      SelectionType::Multiple(sds) => {
                        for sd in sds {
                          let original_vector =
                            Vector2::new(a.data[*sd].x as f32, a.data[*sd].y as f32);
                          let mouse_offset = (projected_mouse - 4.) - self.lc_pressed_pos;
                          let displacement = original_vector + mouse_offset;
                          a.data[*sd].x = displacement.x.round() as u32;
                          a.data[*sd].y = displacement.y.round() as u32;
                          ad.regen_atlas_texture(&mut dc, t, a).unwrap();
                        }
                      }
                    }
                  }
                }
                LeftClickMode::Control => {
                  self.selecting_rectangle = false;
                  let srec = Rectangle::new(
                    self.lc_pressed_pos.x.min(projected_mouse.x),
                    self.lc_pressed_pos.y.min(projected_mouse.y),
                    (self.lc_pressed_pos.x - projected_mouse.x).abs(),
                    (self.lc_pressed_pos.y - projected_mouse.y).abs(),
                  );
                  let mut selected_datas = Vec::new();
                  for (j, data) in a.data.iter().enumerate() {
                    if Rectangle::new(data.x as f32, data.y as f32, 8., 8.)
                      .check_collision_recs(&srec)
                    {
                      selected_datas.push(j);
                    }
                  }
                  c.selected_data = Some(SelectionType::Multiple(selected_datas))
                }
              }
            } else {
              // CLICKING
              match self.lc_mode {
                LeftClickMode::NoModifiers => {
                  c.selected_data = self.select_single_data_at_position(a, self.lc_pressed_pos);
                }
                _ => {}
              }
            }
          }
        }

        // --- RENDERING ---

        let grid_color = Color::new(255, 255, 255, 50);
        for j in 0..1000 {
          dc.draw_line(0, j * 8, 8000, j * 8, grid_color);
          dc.draw_line(j * 8, 0, j * 8, 8000, grid_color);
        }

        if let Some(at) = &ad.atlas_texture {
          dc.draw_texture(&at, 0, 0, Color::WHITE);

          if !self.moving {
            if let Some(st) = &c.selected_data {
              const LINE_WIDTH: f32 = 0.5;
              match st {
                SelectionType::Single(sd) => {
                  dc.draw_rectangle_lines_ex(
                    Rectangle::new(
                      a.data[*sd].x as f32 - LINE_WIDTH,
                      a.data[*sd].y as f32 - LINE_WIDTH,
                      8. + 2. * LINE_WIDTH,
                      8. + 2. * LINE_WIDTH,
                    ),
                    LINE_WIDTH,
                    Color::WHITE,
                  );
                }
                SelectionType::Multiple(sds) => {
                  for sd in sds {
                    dc.draw_rectangle_lines_ex(
                      Rectangle::new(
                        a.data[*sd].x as f32 - LINE_WIDTH,
                        a.data[*sd].y as f32 - LINE_WIDTH,
                        8. + 2. * LINE_WIDTH,
                        8. + 2. * LINE_WIDTH,
                      ),
                      LINE_WIDTH,
                      Color::WHITE,
                    );
                  }
                }
              }
            }
          }

          if self.moving {
            if let Some(st) = &c.selected_data {
              match st {
                SelectionType::Single(sd) => {
                  let dest_rec = Rectangle::new(
                    (projected_mouse.x - 4.).round(),
                    (projected_mouse.y - 4.).round(),
                    8.,
                    8.,
                  );

                  dc.draw_rectangle_rec(dest_rec, Color::new(0, 0, 0, 100));

                  dc.draw_texture_pro(
                    &ad.binary_texture,
                    Rectangle::new(
                      (a.data[*sd].chr_index % 16) as f32 * 8.,
                      (a.data[*sd].chr_index / 16) as f32 * 8.,
                      8.,
                      8.,
                    ),
                    dest_rec,
                    Vector2::zero(),
                    0.,
                    Color::WHITE,
                  );
                }
                SelectionType::Multiple(sds) => {
                  for sd in sds {
                    let original_vector = Vector2::new(a.data[*sd].x as f32, a.data[*sd].y as f32);
                    let mouse_offset = (projected_mouse - 4.) - self.lc_pressed_pos;
                    let displacement = original_vector + mouse_offset;

                    let dest_rec =
                      Rectangle::new(displacement.x.round(), displacement.y.round(), 8., 8.);

                    dc.draw_rectangle_rec(dest_rec, Color::new(0, 0, 0, 100));

                    dc.draw_texture_pro(
                      &ad.binary_texture,
                      Rectangle::new(
                        (a.data[*sd].chr_index % 16) as f32 * 8.,
                        (a.data[*sd].chr_index / 16) as f32 * 8.,
                        8.,
                        8.,
                      ),
                      dest_rec,
                      Vector2::zero(),
                      0.,
                      Color::WHITE,
                    );
                  }
                }
              }
            }
          }

          if self.selecting_rectangle {
            let srec = Rectangle::new(
              self.lc_pressed_pos.x.min(projected_mouse.x),
              self.lc_pressed_pos.y.min(projected_mouse.y),
              (self.lc_pressed_pos.x - projected_mouse.x).abs(),
              (self.lc_pressed_pos.y - projected_mouse.y).abs(),
            );

            dc.draw_rectangle_rec(srec, Color::new(255, 255, 255, 128));
          }
        }
      }
    }

    _r
  }
}
