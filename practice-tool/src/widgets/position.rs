use std::fmt::Write;

use libsekiro::memedit::PointerChain;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::nudge_position::NudgePositionStorage;
use practice_tool_core::widgets::position::{Position, PositionStorage};
use practice_tool_core::widgets::Widget;

pub(super) struct SavePosition {
    ptr: PointerChain<[f32; 4]>,
    saved_position: [f32; 4],
    label_current: String,
    label_stored: String,
    valid: bool,
    nudge: f32,
}

impl SavePosition {
    pub(super) fn new(ptr: PointerChain<[f32; 4]>, nudge: f32) -> Self {
        Self {
            ptr,
            saved_position: [0.0; 4],
            label_current: String::new(),
            label_stored: String::new(),
            valid: false,
            nudge,
        }
    }
}

impl PositionStorage for SavePosition {
    fn save(&mut self) {
        if let Some(pos) = self.ptr.read() {
            self.saved_position = pos;
            self.valid = true;
        } else {
            self.valid = false;
        }
    }

    fn load(&mut self) {
        self.ptr.write(self.saved_position);
    }

    fn display_current(&mut self) -> &str {
        self.label_current.clear();

        let pos = self.ptr.read();

        let (read_pos, valid) = if let Some(pos) = pos { (pos, true) } else { ([0f32; 4], false) };

        self.valid = valid;

        write!(
            self.label_current,
            "{:7.1} {:7.1} {:7.1} {:7.1}",
            read_pos[0], read_pos[1], read_pos[2], read_pos[3]
        )
        .ok();

        &self.label_current
    }

    fn display_stored(&mut self) -> &str {
        self.label_stored.clear();

        let [x, y, z, a] = self.saved_position;

        write!(self.label_stored, "{:7.1} {:7.1} {:7.1} {:7.1}", x, y, z, a).ok();

        &self.label_stored
    }

    fn is_valid(&self) -> bool {
        self.valid
    }
}

impl NudgePositionStorage for SavePosition {
    fn nudge_up(&mut self) {
        if let Some([x, y, z, w]) = self.ptr.read() {
            self.ptr.write([x, y + self.nudge, z, w]);
        }
    }

    fn nudge_down(&mut self) {
        if let Some([x, y, z, w]) = self.ptr.read() {
            self.ptr.write([x, y - self.nudge, z, w]);
        }
    }
}

pub(crate) fn save_position(
    ptr: PointerChain<[f32; 4]>,
    key_load: Option<Key>,
    key_save: Option<Key>,
) -> Box<dyn Widget> {
    Box::new(Position::new(SavePosition::new(ptr, 0.0), key_load, key_save))
}
