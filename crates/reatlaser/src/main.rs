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
use crate::pane::selected::SelectedPanel;
use crate::pane::selected::SelectedPanelMessage;
use crate::window::Window;
use crate::window::atlas_creator::AtlasCreator;
use crate::window::color_picker::ColorPicker;

pub(crate) mod atlas;
pub(crate) mod editor;
pub(crate) mod pane;
pub(crate) mod window;

pub(crate) enum SelectionType {
  Single(usize),
  Multiple(Vec<usize>),
}

pub(crate) struct Context {
  atlas: Option<Atlas>,
  atlas_display: Option<AtlasDisplay>,
  selected_data: Option<SelectionType>,

  default_colors: [usize; 3],
  default_background_color: Option<usize>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      atlas: None,
      atlas_display: None,
      selected_data: None,
      default_colors: [0, 0, 0],
      default_background_color: None,
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

  let style_loc = dirs::config_dir()
    .unwrap()
    .join(Path::new("reatlaser/style.rgs"));

  rl.gui_load_style(style_loc.to_str().unwrap());

  let mut context = Context::new();

  let mut editor = Editor::init();

  let mut file_panel = FilePanel::init();
  let mut picker_panel = PickerPanel::init();
  let mut selected_panel = SelectedPanel::init();

  let mut atlas_context_window = AtlasCreator::init();
  let mut color_picker_window = ColorPicker::init();

  while !rl.window_should_close() {
    {
      // --- DRAW ---
      let mut d = rl.begin_drawing(&thread);

      d.clear_background(Color::get_color(
        d.gui_get_style(DEFAULT, BACKGROUND_COLOR) as u32
      ));

      // --- EDITOR ---
      {
        let i = !atlas_context_window.is_opened() && !color_picker_window.is_opened();
        let mut no_passthrough_rects = vec![];
        if file_panel.is_opened(&context) {
          no_passthrough_rects.push(file_panel.get_rect(&mut d));
        }
        if picker_panel.is_opened(&context) {
          no_passthrough_rects.push(picker_panel.get_rect(&mut d));
        }
        if selected_panel.is_opened(&context) {
          no_passthrough_rects.push(selected_panel.get_rect(&mut d));
        }
        editor.display(&mut d, &thread, &mut context, no_passthrough_rects, i);
      }

      // --- FILE PANEL ---
      if file_panel.is_opened(&context) {
        let i = !atlas_context_window.is_opened() && !color_picker_window.is_opened();
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
        let i = !atlas_context_window.is_opened() && !color_picker_window.is_opened();
        picker_panel.display(&mut d, &thread, &mut context, i);
      }

      // --- SELECTED ---
      if selected_panel.is_opened(&context) {
        let i = !atlas_context_window.is_opened() && !color_picker_window.is_opened();
        let messages = selected_panel.display(&mut d, &thread, &mut context, i);
        for m in messages {
          match m {
            SelectedPanelMessage::OpenColorPicker(tc) => {
              color_picker_window.set_opened(true);
              color_picker_window.set_data(tc);
            }
          }
        }
      }

      // --- ATLAS CREATION MENU ---
      if atlas_context_window.is_opened() {
        let i = !color_picker_window.is_opened();
        atlas_context_window.display(&mut d, &thread, &mut context, i);
      }

      if color_picker_window.is_opened() {
        color_picker_window.display(&mut d, &thread, &mut context, true);
      }
    }
  }
}
