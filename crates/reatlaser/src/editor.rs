use raylib::prelude::*;

use crate::Context;

pub struct Editor {
  camera: Camera2D,

  moving_data: Option<usize>,
  moving_initial_click: Vector2,
  moving: bool,
}

pub enum EditorMessages {}

impl Editor {
  pub fn init() -> Self {
    Self {
      camera: Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.,
        zoom: 3.,
      },

      moving_data: None,
      moving_initial_click: Vector2::zero(),
      moving: false,
    }
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

          if dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            c.selected_data = None;
            for (j, data) in a.data.iter().enumerate() {
              if Rectangle::new(data.x as f32, data.y as f32, 8., 8.)
                .check_collision_point_rec(projected_mouse)
              {
                if self.moving_data.is_none() {
                  self.moving_data = Some(j);
                  self.moving_initial_click = projected_mouse;
                  self.moving = false;
                }
                if c.selected_data.is_none() {
                  c.selected_data = Some(j);
                }
              }
            }
            if self.moving_data.is_some()
              && !self.moving
              && projected_mouse.distance_to(self.moving_initial_click) > 1.5
            {
              self.moving = true;
            }
          } else {
            if self.moving {
              if let Some(sd) = &self.moving_data {
                a.data[*sd].x = (projected_mouse.x - 4.).round() as u32;
                a.data[*sd].y = (projected_mouse.y - 4.).round() as u32;
                self.moving_data = None;
                self.moving = false;
                ad.regen_atlas_texture(&mut dc, t, a).unwrap();
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
          if self.moving && self.moving_data.is_some() {
            dc.draw_rectangle_rec(
              Rectangle::new(
                (projected_mouse.x - 4.).round(),
                (projected_mouse.y - 4.).round(),
                8.,
                8.,
              ),
              Color::new(255, 255, 255, 100),
            );
          }
        }
      }
    }

    _r
  }
}
