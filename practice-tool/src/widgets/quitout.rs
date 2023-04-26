use hudhook::imgui;
use libsekiro::prelude::*;
use practice_tool_utils::widgets::{scaling_factor, Widget, BUTTON_HEIGHT, BUTTON_WIDTH};
use practice_tool_utils::KeyState;

#[derive(Debug)]
pub(crate) struct Quitout {
    label: String,
    ptr: PointerChain<u8>,
    hotkey: KeyState,
}

impl Quitout {
    pub(crate) fn new(ptr: PointerChain<u8>, hotkey: KeyState) -> Self {
        Quitout { label: format!("Quitout ({})", hotkey), ptr, hotkey }
    }
}

impl Widget for Quitout {
    fn render(&mut self, ui: &imgui::Ui) {
        let scale = scaling_factor(ui);

        if ui.button_with_size(&self.label, [BUTTON_WIDTH * scale, BUTTON_HEIGHT]) {
            self.ptr.write(1);
        }
    }

    fn interact(&mut self, ui: &imgui::Ui) {
        if self.hotkey.keyup(ui) {
            self.ptr.write(1);
        }
    }
}
