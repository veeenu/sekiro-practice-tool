use hudhook::imgui;
use libsekiro::prelude::*;
use practice_tool_utils::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use practice_tool_utils::KeyState;

#[derive(Debug)]
pub(crate) struct SavePosition {
    position: PointerChain<[f32; 4]>,
    hotkey: KeyState,
    modifier: KeyState,
    saved_position: [f32; 4],
}

impl SavePosition {
    pub(crate) fn new(
        position: PointerChain<[f32; 4]>,
        hotkey: KeyState,
        modifier: KeyState,
    ) -> Self {
        SavePosition { position, hotkey, modifier, saved_position: [0f32; 4] }
    }

    fn save_position(&mut self) {
        if let Some([x, y, z, angle]) = self.position.read() {
            self.saved_position = [x, y, z, angle];
        }
    }

    fn load_position(&mut self) {
        if self.position.read().is_some() {
            self.position.write(self.saved_position);
        }
    }
}

impl Widget for SavePosition {
    fn render(&mut self, ui: &imgui::Ui) {
        let saved_pos = self.saved_position;

        let (read_pos, valid) = if let Some([x, y, z, angle]) = self.position.read() {
            ([x, y, z, angle], true)
        } else {
            ([0f32; 4], false)
        };

        let _token = ui.begin_disabled(!valid);
        let button_width = BUTTON_WIDTH * scaling_factor(ui);

        if ui.button_with_size(format!("Load ({})", self.hotkey), [
            button_width * 0.33 - 4.,
            BUTTON_HEIGHT,
        ]) {
            self.load_position();
        }
        ui.same_line();
        if ui.button_with_size(format!("Save ({} + {})", self.modifier, self.hotkey), [
            button_width * 0.67 - 4.,
            BUTTON_HEIGHT,
        ]) {
            self.save_position();
        }
        ui.text(format!(
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            read_pos[0], read_pos[1], read_pos[2], read_pos[3]
        ));
        ui.text(format!(
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            saved_pos[0], saved_pos[1], saved_pos[2], saved_pos[3],
        ));
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        let key_up = self.hotkey.keyup(ui);
        let mod_down = self.modifier.is_key_down(ui);

        if key_up && mod_down {
            self.save_position();
        } else if key_up {
            self.load_position();
        }
    }
}
