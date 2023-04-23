use std::ptr::null_mut;

use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::codegen::base_addresses::BaseAddresses;
use crate::memedit::{Bitflag, PointerChain};
use crate::{bitflag, pointer_chain};

pub struct Pointers {
    pub position: PointerChain<[f32; 3]>,
    pub quitout: Bitflag<u8>,
    pub render_world: Bitflag<u8>,
    pub debug_render0: Bitflag<u8>,
    pub debug_render1: Bitflag<u8>,
    pub debug_render8: Bitflag<u8>,
    pub player_hide: Bitflag<u8>,
    pub all_no_update_ai: Bitflag<u8>,
    pub all_no_damage: Bitflag<u8>,
    pub no_goods_consume: Bitflag<u8>,
    pub no_resource_item_consume: Bitflag<u8>,
}

impl Default for Pointers {
    fn default() -> Self {
        Self::new()
    }
}

impl Pointers {
    pub fn new() -> Self {
        let base_module_address =
            unsafe { GetModuleHandleA(PCSTR(null_mut())).unwrap() }.0 as usize;
        let _base_addresses = BaseAddresses::from(*crate::version::VERSION)
            .with_module_base_addr(base_module_address);

        let pos_base = 0x143B67DF0;
        let quitout_base = 0x143B55048;
        let debug_render_base = 0x143B65BC0;
        let render_world_base = 0x1439007C8;
        let debug_flags_base = 0x143B67F00;

        Pointers {
            position: pointer_chain!(pos_base, 0x48, 0x28, 0x80),
            quitout: bitflag!(0b1; quitout_base, 0x23C),
            render_world: bitflag!(0b1; render_world_base),
            debug_render0: bitflag!(0b1; debug_render_base),
            debug_render1: bitflag!(0b1; debug_render_base + 1),
            debug_render8: bitflag!(0b1; debug_render_base + 0xC),
            player_hide: bitflag!(0b1; debug_flags_base + 0x5F),
            all_no_update_ai: bitflag!(0b1; debug_flags_base + 0x66),
            all_no_damage: bitflag!(0b1; debug_flags_base + 0x62),
            no_goods_consume: bitflag!(0b1; debug_flags_base + 0x59),
            no_resource_item_consume: bitflag!(0b1; debug_flags_base + 0x5a),
        }
    }
}
