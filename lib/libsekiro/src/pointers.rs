use std::ptr::null_mut;

use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

use crate::codegen::base_addresses::BaseAddresses;
use crate::memedit::{Bitflag, PointerChain};
use crate::prelude::{Version, VERSION};
use crate::{bitflag, pointer_chain};

pub struct Pointers {
    pub position: PointerChain<[f32; 4]>,
    pub quitout: PointerChain<u8>,
    pub show_cursor: Bitflag<u8>,
    pub igt: PointerChain<u32>,

    pub fps: PointerChain<f32>,

    pub anim_speed: PointerChain<f32>,

    pub render_world: Bitflag<u8>,
    pub render_objects: Bitflag<u8>,
    pub render_mobs: Bitflag<u8>,
    pub render_effects: Bitflag<u8>,

    pub debug_render0: Bitflag<u8>,
    pub debug_render1: Bitflag<u8>,
    pub debug_render2: Bitflag<u8>,
    pub debug_render3: Bitflag<u8>,
    pub debug_render4: Bitflag<u8>,
    pub debug_render5: Bitflag<u8>,
    pub debug_render6: Bitflag<u8>,
    pub debug_render7: Bitflag<u8>,

    pub debug_color: PointerChain<i32>,

    pub debug_show: Bitflag<u8>,
    pub grapple_debug_path: Bitflag<u8>,
    pub grapple_debug_col: Bitflag<u8>,

    pub player_no_goods_consume: Bitflag<u8>,
    pub player_no_resource_item_consume: Bitflag<u8>,
    pub player_no_revival_consume: Bitflag<u8>,
    pub player_hide: Bitflag<u8>,
    pub player_silence: Bitflag<u8>,
    pub player_no_dead: Bitflag<u8>,
    pub player_exterminate: Bitflag<u8>,
    pub player_exterminate_stamina: Bitflag<u8>,

    pub all_no_dead: Bitflag<u8>,
    pub all_no_damage: Bitflag<u8>,
    pub all_no_hit: Bitflag<u8>,
    pub all_no_attack: Bitflag<u8>,
    pub all_no_move: Bitflag<u8>,
    pub all_no_update_ai: Bitflag<u8>,
    pub all_no_stamina_consume: Bitflag<u8>,
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
        let base_addresses = BaseAddresses::from(*crate::version::VERSION)
            .with_module_base_addr(base_module_address);

        let BaseAddresses {
            quitout,
            render_world,
            debug_render,
            igt,
            player_position,
            debug_flags,
            show_cursor,
            debug_show,
            grapple_debug,
            fps,
            ..
        } = base_addresses;

        let offs_player_no_dead: isize = match *VERSION {
            Version::V1_02_0 | Version::V1_03_0 | Version::V1_04_0 => 33,
            Version::V1_05_0 | Version::V1_06_0 => -3,
        };

        let offs_player_exterminate: isize = match *VERSION {
            Version::V1_02_0 | Version::V1_03_0 | Version::V1_04_0 => 52,
            Version::V1_05_0 | Version::V1_06_0 => -2,
        };

        let offs_player_exterminate_stamina: isize = match *VERSION {
            Version::V1_02_0 | Version::V1_03_0 | Version::V1_04_0 => -1,
            Version::V1_05_0 | Version::V1_06_0 => -1,
        };

        let offs_grapple_debug: usize = match *VERSION {
            Version::V1_02_0 | Version::V1_03_0 | Version::V1_04_0 => 0xEC8,
            Version::V1_05_0 | Version::V1_06_0 => 0xF68,
        };

        Pointers {
            position: pointer_chain!(player_position, 0x48, 0x28, 0x80),
            quitout: pointer_chain!(quitout, 0x23C),
            show_cursor: bitflag!(0b1; show_cursor),
            igt: pointer_chain!(igt, 0x9C),

            fps: pointer_chain!(fps, 0x2BC),

            anim_speed: pointer_chain!(player_position, 0x48, 0x28, 0xA40, 0x4C0, 0x250, 0x10, 0xD00),

            render_world: bitflag!(0b1; render_world),
            render_objects: bitflag!(0b1; render_world+1),
            render_mobs: bitflag!(0b1; render_world+2),
            render_effects: bitflag!(0b1; render_world+3),
            debug_render0: bitflag!(0b1; debug_render),
            debug_render1: bitflag!(0b1; debug_render + 1),
            debug_render2: bitflag!(0b1; debug_render + 2),
            debug_render3: bitflag!(0b1; debug_render + 5),
            debug_render4: bitflag!(0b1; debug_render + 6),
            debug_render5: bitflag!(0b1; debug_render + 7),
            debug_render6: bitflag!(0b1; debug_render + 8),
            debug_render7: bitflag!(0b1; debug_render + 9),

            debug_color: pointer_chain!(debug_render + 0xC),

            debug_show: bitflag!(0b1; debug_show, 0x6F),
            grapple_debug_path: bitflag!(0b1; grapple_debug, 0xC8, 0x20, offs_grapple_debug),
            grapple_debug_col: bitflag!(0b1; grapple_debug, 0xC8, 0x20, offs_grapple_debug + 0x2),

            player_no_goods_consume: bitflag!(0b1; debug_flags),
            player_no_resource_item_consume: bitflag!(0b1; debug_flags + 1),
            player_no_revival_consume: bitflag!(0b1; debug_flags + 2),
            player_hide: bitflag!(0b1; debug_flags + 6),
            player_silence: bitflag!(0b1; debug_flags + 7),
            player_no_dead: bitflag!(0b1; debug_flags.saturating_add_signed(offs_player_no_dead)),
            player_exterminate: bitflag!(0b1; debug_flags.saturating_add_signed(offs_player_exterminate)),
            player_exterminate_stamina: bitflag!(0b1; debug_flags.saturating_add_signed(offs_player_exterminate_stamina)),
            all_no_dead: bitflag!(0b1; debug_flags + 8),
            all_no_damage: bitflag!(0b1; debug_flags + 9),
            all_no_hit: bitflag!(0b1; debug_flags + 10),
            all_no_attack: bitflag!(0b1; debug_flags + 11),
            all_no_move: bitflag!(0b1; debug_flags + 12),
            all_no_update_ai: bitflag!(0b1; debug_flags + 13),
            all_no_stamina_consume: bitflag!(0b1; debug_flags + 20),
        }
    }
}
