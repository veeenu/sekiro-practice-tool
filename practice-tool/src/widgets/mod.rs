use std::ops::Deref;

use libsekiro::prelude::PointerChain;

pub mod flag;
pub mod nudge_pos;
pub mod position;
pub mod quitout;
pub mod savefile_manager;

pub struct Position(PointerChain<[f32; 3]>);

impl Deref for Position {
    type Target = PointerChain<[f32; 3]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
