use raylib::prelude::*;

use crate::Context;

pub struct Editor {
  camera: Camera2D,

  selected_data: Option<usize>,
}

pub enum EditorMessages {}

impl Editor {
  pub fn init() -> Self {
    Self {
      camera: Camera2D {
        offset: Vector2::zero(),
        target: Vector2::zero(),
        rotation: 0.,
        zoom: 1.,
      },

      selected_data: None,
    }
  }

  pub fn display(
    &mut self,
    d: &mut RaylibDrawHandle,
    _t: &RaylibThread,
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

          if dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE) {
            self.camera.target -= dc.get_mouse_delta() / self.camera.zoom;
          }

          if dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) && self.selected_data.is_none()
          {
            for (j, data) in a.data.iter().enumerate() {
              if Rectangle::new(data.x as f32 * 8., data.y as f32 * 8., 8., 8.)
                .check_collision_point_rec(projected_mouse)
              {
                self.selected_data = Some(j);
              }
            }
          }
          if !dc.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(sd) = &self.selected_data {
              a.data[*sd].x = (projected_mouse.x / 8.).floor() as u32;
              a.data[*sd].y = (projected_mouse.y / 8.).floor() as u32;
              self.selected_data = None;
            }
          }
        }

        // --- RENDERING ---
        for (j, data) in a.data.iter().enumerate() {
          let texture_width = ad.binary_texture.width as f32;

          let source_size = texture_width / 16.;

          let source_rec = Rectangle::new(
            (data.chr_index % 16) as f32 * source_size,
            (data.chr_index / 16) as f32 * source_size,
            source_size,
            source_size,
          );

          let dst_rec = match self.selected_data {
            None => Rectangle::new(data.x as f32 * 8., data.y as f32 * 8., 8., 8.),
            Some(id) => match id == j {
              false => Rectangle::new(data.x as f32 * 8., data.y as f32 * 8., 8., 8.),
              true => Rectangle::new(projected_mouse.x - 4., projected_mouse.y - 4., 8., 8.),
            },
          };

          dc.draw_texture_pro(
            &ad.binary_texture,
            source_rec,
            dst_rec,
            Vector2::zero(),
            0.,
            Color::WHITE,
          );
        }
      }
    }

    _r
  }
}
