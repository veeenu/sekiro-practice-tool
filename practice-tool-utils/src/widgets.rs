use std::sync::Arc;

use hudhook::imgui;
use parking_lot::Mutex;

pub const BUTTON_WIDTH: f32 = 320.;
pub const BUTTON_HEIGHT: f32 = 0.;
pub const MODAL_BACKGROUND: [f32; 4] = [0.1, 0.1, 0.1, 0.5];

pub trait Widget: Send + Sync + std::fmt::Debug {
    fn render(&mut self, _ui: &imgui::Ui);
    fn interact(&mut self, _ui: &imgui::Ui) {}
    fn interact_ui(&mut self, _ui: &imgui::Ui) {}

    fn enter(&self, _ui: &imgui::Ui) -> Option<Arc<Mutex<Box<dyn Widget>>>> {
        None
    }
    fn exit(&self, _ui: &imgui::Ui) {}
    fn cursor_down(&mut self) {}
    fn cursor_up(&mut self) {}

    fn want_enter(&mut self) -> bool {
        false
    }
    fn want_exit(&mut self) -> bool {
        false
    }
    fn log(&mut self) -> Option<Vec<String>> {
        None
    }
}

pub fn scaling_factor(ui: &imgui::Ui) -> f32 {
    let width = ui.io().display_size[0];
    if width > 2000. {
        1. + 1. / 3.
    } else if width > 1200. {
        1.
    } else {
        2. / 3.
    }
}
