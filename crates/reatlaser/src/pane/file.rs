use std::fs::{read_to_string, write};

use crate::{Context, atlas::AtlasDisplay, pane::Pane};
use chr_reatlas::atlas::Atlas;
use raylib::prelude::*;
use rfd::FileDialog;

pub struct FilePanel {}

pub enum FilePanelMessage {
  OpenAtlasCreator,
}

impl Pane<Context, FilePanelMessage> for FilePanel {
  fn init() -> Self {
    Self {}
  }

  fn get_rect(&self, _d: &raylib::prelude::RaylibDrawHandle) -> raylib::prelude::Rectangle {
    Rectangle::new(10., 10., 95., 125.)
  }

  fn display(
    &mut self,
    d: &mut RaylibDrawHandle,
    t: &RaylibThread,
    c: &mut Context,
    i: bool,
  ) -> Vec<FilePanelMessage> {
    let mut r = Vec::new();

    d.gui_panel(self.get_rect(d), "reatlaser");

    if d.gui_button(Rectangle::new(20., 45., 75., 20.), "New") && i {
      r.push(FilePanelMessage::OpenAtlasCreator);
    }
    if d.gui_button(Rectangle::new(20., 75., 75., 20.), "Load") && i {
      let file_option = FileDialog::new().add_filter("toml", &["toml"]).pick_file();
      if let Some(file_path) = file_option {
        let file_content = read_to_string(file_path).unwrap();
        let new_atlas: Atlas = toml::from_str(&file_content).unwrap();
        let atlas_display = AtlasDisplay::from_atlas(d, t, &new_atlas).unwrap();
        c.atlas = Some(new_atlas);
        c.atlas_display = Some(atlas_display);
      }
    }
    if let Some(a) = &c.atlas {
      if d.gui_button(Rectangle::new(20., 105., 75., 20.), "Save") && i {
        let file_option = FileDialog::new().add_filter("toml", &["toml"]).save_file();
        if let Some(file_path) = file_option {
          let toml_string = toml::to_string(&a).unwrap();
          write(file_path, toml_string).unwrap();
        }
      }
    }

    r
  }
}
