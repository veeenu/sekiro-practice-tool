use hudhook::imgui;
use libsekiro::prelude::*;
use practice_tool_utils::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use practice_tool_utils::KeyState;

#[derive(Debug)]
pub(crate) struct NudgePosition {
    position: PointerChain<[f32; 4]>,
    nudge: f32,
    nudge_up: Option<KeyState>,
    nudge_down: Option<KeyState>,
    nudge_up_label: String,
    nudge_down_label: String,
}

impl NudgePosition {
    pub(crate) fn new(
        position: PointerChain<[f32; 4]>,
        nudge: f32,
        nudge_up: Option<KeyState>,
        nudge_down: Option<KeyState>,
    ) -> Self {
        let nudge_up_label = if let Some(k) = &nudge_up {
            format!("Nudge up ({})", k)
        } else {
            "Nudge up".to_string()
        };
        let nudge_down_label = if let Some(k) = &nudge_down {
            format!("Nudge down ({})", k)
        } else {
            "Nudge down".to_string()
        };
        NudgePosition { position, nudge, nudge_up, nudge_down, nudge_up_label, nudge_down_label }
    }

    fn do_nudge_up(&mut self) {
        if let Some([x, y, z, angle]) = self.position.read() {
            self.position.write([x, y + self.nudge, z, angle]);
        }
    }

    fn do_nudge_down(&mut self) {
        if let Some([x, y, z, angle]) = self.position.read() {
            self.position.write([x, y - self.nudge, z, angle]);
        }
    }
}

impl Widget for NudgePosition {
    fn render(&mut self, ui: &imgui::Ui) {
        let valid = self.position.eval().is_some();
        let _token = ui.begin_disabled(!valid);

        let button_width = BUTTON_WIDTH * scaling_factor(ui);

        if ui.button_with_size(&self.nudge_up_label, [button_width * 0.5 - 4., BUTTON_HEIGHT]) {
            self.do_nudge_up();
        }
        ui.same_line();
        if ui.button_with_size(&self.nudge_down_label, [button_width * 0.5 - 4., BUTTON_HEIGHT]) {
            self.do_nudge_down();
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if let Some(true) = self.nudge_up.as_ref().map(|c| c.is_key_down(ui)) {
            self.do_nudge_up();
        } else if let Some(true) = self.nudge_down.as_ref().map(|c| c.is_key_down(ui)) {
            self.do_nudge_down();
        }
    }
}
