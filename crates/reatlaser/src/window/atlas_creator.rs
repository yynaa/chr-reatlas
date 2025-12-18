use anyhow::Result;
use chr_reatlas::atlas::Atlas;
use raylib::prelude::*;

use crate::{Context, atlas::AtlasDisplay, make_text_input, window::Window};

pub struct AtlasCreator {
  opened: bool,

  text_box_edits: Vec<bool>,
  text_box_buffers: Vec<String>,

  error_message: String,
}

pub enum AtlasCreatorMessage {}

impl AtlasCreator {
  fn create_atlas(
    &mut self,
    d: &mut RaylibDrawHandle,
    t: &RaylibThread,
    c: &mut Context,
  ) -> Result<()> {
    let new_atlas = Atlas {
      binary: self.text_box_buffers[0].clone(),
      palette: self.text_box_buffers[1].clone(),
      start: u64::from_str_radix(&self.text_box_buffers[2], 16)?,
      length: usize::from_str_radix(&self.text_box_buffers[3], 16)? * 0x10,
      data: Vec::new(),
    };

    let atlas_display = AtlasDisplay::from_atlas(d, t, &new_atlas)?;

    c.atlas = Some(new_atlas);
    c.atlas_display = Some(atlas_display);

    Ok(())
  }
}

impl Window<Context, AtlasCreatorMessage> for AtlasCreator {
  fn init() -> Self {
    Self {
      opened: false,

      text_box_edits: vec![false, false, false, false],
      text_box_buffers: vec![String::new(), String::new(), String::new(), String::new()],

      error_message: String::new(),
    }
  }

  fn get_rect(&self, d: &RaylibDrawHandle) -> Rectangle {
    let width = d.get_screen_width();
    let height = d.get_screen_height();

    let window_width = 300.;
    let window_height = 250.;

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
  ) -> Vec<AtlasCreatorMessage> {
    let mut _r = Vec::new();

    if self.opened {
      let width = d.get_screen_width();
      let height = d.get_screen_height();

      d.draw_rectangle(0, 0, width, height, Color::new(0, 0, 0, 128));

      let r = self.get_rect(d);

      if d.gui_window_box(r, "creating new atlas") {
        self.opened = false;
      }

      if let Some(ch) = d.get_char_pressed() {
        for (j, edit) in self.text_box_edits.iter().enumerate() {
          if *edit && i {
            self.text_box_buffers[j].push(ch);
          }
        }
      }

      if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) && i {
        for (j, edit) in self.text_box_edits.iter().enumerate() {
          if *edit {
            self.text_box_buffers[j].pop();
          }
        }
      }

      d.gui_label(
        Rectangle::new(r.x + 10., r.y + 35., r.width - 20., 10.),
        "Relative path to binary",
      );
      make_text_input!(
        d,
        self.text_box_buffers,
        self.text_box_edits,
        0,
        Rectangle::new(r.x + 10., r.y + 50., r.width - 20., 20.),
        i
      );

      d.gui_label(
        Rectangle::new(r.x + 10., r.y + 75., r.width - 20., 10.),
        "Relative path to palette",
      );
      make_text_input!(
        d,
        self.text_box_buffers,
        self.text_box_edits,
        1,
        Rectangle::new(r.x + 10., r.y + 90., r.width - 20., 20.),
        i
      );

      d.gui_label(
        Rectangle::new(r.x + 10., r.y + 115., r.width - 20., 10.),
        "Start address (hexadecimal)",
      );
      make_text_input!(
        d,
        self.text_box_buffers,
        self.text_box_edits,
        2,
        Rectangle::new(r.x + 10., r.y + 130., r.width - 20., 20.),
        i
      );

      d.gui_label(
        Rectangle::new(r.x + 10., r.y + 155., r.width - 20., 10.),
        "Length (hexadecimal, in tiles)",
      );
      make_text_input!(
        d,
        self.text_box_buffers,
        self.text_box_edits,
        3,
        Rectangle::new(r.x + 10., r.y + 170., r.width - 20., 20.),
        i
      );

      d.gui_label(
        Rectangle::new(r.x + 10., r.y + r.height - 45., r.width - 20., 10.),
        &self.error_message,
      );
      if d.gui_button(
        Rectangle::new(r.x + r.width / 2. - 50., r.y + r.height - 30., 100., 20.),
        "Create Atlas",
      ) && i
      {
        if let Err(s) = self.create_atlas(d, t, c) {
          self.error_message = s.to_string();
        } else {
          self.opened = false;
        }
      }
    }

    _r
  }

  fn is_opened(&self) -> bool {
    self.opened
  }

  fn set_opened(&mut self, opened: bool) {
    self.opened = opened;
    if opened {
      self.text_box_buffers.fill(String::new());
    }
  }
}
