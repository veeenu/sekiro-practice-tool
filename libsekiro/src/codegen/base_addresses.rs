// **********************************
// *** AUTOGENERATED, DO NOT EDIT ***
// **********************************
#[derive(Debug)]
pub struct BaseAddresses {
    pub quitout: usize,
    pub render_world: usize,
    pub debug_render: usize,
    pub igt: usize,
    pub player_position: usize,
    pub debug_flags: usize,
    pub show_cursor: usize,
    pub no_logo: usize,
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
            quitout: self.quitout + base,
            render_world: self.render_world + base,
            debug_render: self.debug_render + base,
            igt: self.igt + base,
            player_position: self.player_position + base,
            debug_flags: self.debug_flags + base,
            show_cursor: self.show_cursor + base,
            no_logo: self.no_logo + base,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    V1_02_0,
    V1_03_0,
    V1_04_0,
    V1_05_0,
    V1_06_0,
}

impl From<(u32, u32, u32)> for Version {
    fn from(v: (u32, u32, u32)) -> Self {
        match v {
            (1, 2, 0) => Version::V1_02_0,
            (1, 3, 0) => Version::V1_03_0,
            (1, 4, 0) => Version::V1_04_0,
            (1, 5, 0) => Version::V1_05_0,
            (1, 6, 0) => Version::V1_06_0,
            (maj, min, patch) => {
                tracing::error!("Unrecognized version {maj}.{min:02}.{patch}");
                panic!()
            },
        }
    }
}

impl Version {
    pub fn tuple(&self) -> (u8, u8, u8) {
        match self {
            Version::V1_02_0 => (1, 2, 0),
            Version::V1_03_0 => (1, 3, 0),
            Version::V1_04_0 => (1, 4, 0),
            Version::V1_05_0 => (1, 5, 0),
            Version::V1_06_0 => (1, 6, 0),
        }
    }
}

impl From<Version> for BaseAddresses {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_02_0 => BASE_ADDRESSES_1_02_0,
            Version::V1_03_0 => BASE_ADDRESSES_1_03_0,
            Version::V1_04_0 => BASE_ADDRESSES_1_04_0,
            Version::V1_05_0 => BASE_ADDRESSES_1_05_0,
            Version::V1_06_0 => BASE_ADDRESSES_1_06_0,
        }
    }
}

pub const BASE_ADDRESSES_1_02_0: BaseAddresses = BaseAddresses {
    quitout: 0x3b55048,
    render_world: 0x39007c8,
    debug_render: 0x3b65bc0,
    igt: 0x3b47cf0,
    player_position: 0x3b67df0,
    debug_flags: 0x3b67f59,
    show_cursor: 0x3b77048,
    no_logo: 0xdebf2b,
};

pub const BASE_ADDRESSES_1_03_0: BaseAddresses = BaseAddresses {
    quitout: 0x3b56088,
    render_world: 0x39017c8,
    debug_render: 0x3b66c00,
    igt: 0x3b48d30,
    player_position: 0x3b68e30,
    debug_flags: 0x3b68f99,
    show_cursor: 0x3b78088,
    no_logo: 0xdec85b,
};

pub const BASE_ADDRESSES_1_04_0: BaseAddresses = BaseAddresses {
    quitout: 0x3b56088,
    render_world: 0x39017c8,
    debug_render: 0x3b66c00,
    igt: 0x3b48d30,
    player_position: 0x3b68e30,
    debug_flags: 0x3b68f99,
    show_cursor: 0x3b78088,
    no_logo: 0xdec85b,
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
    quitout: 0x3d67368,
    render_world: 0x3b01838,
    debug_render: 0x3d77f04,
    igt: 0x3d5aa20,
    player_position: 0x3d7a140,
    debug_flags: 0x3d7a2c9,
    show_cursor: 0x3d8986c,
    no_logo: 0xe1b1ab,
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
    quitout: 0x3d67408,
    render_world: 0x3b01838,
    debug_render: 0x3d77fa4,
    igt: 0x3d5aac0,
    player_position: 0x3d7a1e0,
    debug_flags: 0x3d7a369,
    show_cursor: 0x3d8990c,
    no_logo: 0xe1b51b,
};
