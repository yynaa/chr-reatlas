use std::env;
use std::path::Path;

use chr_reatlas::atlas::Atlas;
use raylib::prelude::GuiControl::*;
use raylib::prelude::GuiDefaultProperty::*;
use raylib::prelude::*;

use crate::atlas::AtlasDisplay;
use crate::editor::Editor;
use crate::pane::Pane;
use crate::pane::file::FilePanel;
use crate::pane::file::FilePanelMessage;
use crate::pane::picker::PickerPanel;
use crate::window::Window;
use crate::window::atlas_creator::AtlasCreator;

pub(crate) mod atlas;
pub(crate) mod editor;
pub(crate) mod pane;
pub(crate) mod window;

pub(crate) struct Context {
  atlas: Option<Atlas>,
  atlas_display: Option<AtlasDisplay>,
  selected_data: Option<usize>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      atlas: None,
      atlas_display: None,
      selected_data: None,
    }
  }
}

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(1280, 720)
    .resizable()
    .title("reatlaser")
    .build();

  rl.set_target_fps(60);

  let style_loc = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string()))
    .join("assets/style.rgs");

  rl.gui_load_style(style_loc.to_str().unwrap());

  let mut context = Context::new();

  let mut editor = Editor::init();

  let mut file_panel = FilePanel::init();
  let mut picker_panel = PickerPanel::init();

  let mut atlas_context_window = AtlasCreator::init();

  while !rl.window_should_close() {
    {
      // --- DRAW ---
      let mut d = rl.begin_drawing(&thread);

      d.clear_background(Color::get_color(
        d.gui_get_style(DEFAULT, BACKGROUND_COLOR) as u32
      ));

      // --- EDITOR ---
      {
        let i = !atlas_context_window.is_opened();
        let mut no_passthrough_rects = vec![];
        if file_panel.is_opened(&context) {
          no_passthrough_rects.push(file_panel.get_rect(&mut d));
        }
        if picker_panel.is_opened(&context) {
          no_passthrough_rects.push(picker_panel.get_rect(&mut d));
        }
        editor.display(&mut d, &thread, &mut context, no_passthrough_rects, i);
      }

      // --- FILE PANEL ---
      if file_panel.is_opened(&context) {
        let i = !atlas_context_window.is_opened();
        let messages = file_panel.display(&mut d, &thread, &mut context, i);
        for m in messages {
          match m {
            FilePanelMessage::OpenAtlasCreator => {
              atlas_context_window.set_opened(true);
            }
          }
        }
      }

      // --- PICKER ---
      if picker_panel.is_opened(&context) {
        let i = !atlas_context_window.is_opened();
        picker_panel.display(&mut d, &thread, &mut context, i);
      }

      // --- ATLAS CREATION MENU ---
      {
        atlas_context_window.display(&mut d, &thread, &mut context, true);
      }
    }
  }
}
