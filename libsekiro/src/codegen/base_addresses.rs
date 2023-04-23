// **********************************
// *** AUTOGENERATED, DO NOT EDIT ***
// **********************************
#[derive(Debug)]
pub struct BaseAddresses {
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
        }
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    V1_02_0,
    V1_03_0,
    V1_05_0,
    V1_06_0,
}

impl From<(u32, u32, u32)> for Version {
    fn from(v: (u32, u32, u32)) -> Self {
        match v {
            (1, 2, 0) => Version::V1_02_0,
            (1, 3, 0) => Version::V1_03_0,
            (1, 5, 0) => Version::V1_05_0,
            (1, 6, 0) => Version::V1_06_0,
            (maj, min, patch) => {
                tracing::error!("Unrecognized version {maj}.{min:02}.{patch}");
                panic!()
            }
        }
    }
}

impl From<Version> for BaseAddresses {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_02_0 => BASE_ADDRESSES_1_02_0,
            Version::V1_03_0 => BASE_ADDRESSES_1_03_0,
            Version::V1_05_0 => BASE_ADDRESSES_1_05_0,
            Version::V1_06_0 => BASE_ADDRESSES_1_06_0,
        }
    }
}

pub const BASE_ADDRESSES_1_02_0: BaseAddresses = BaseAddresses {
};

pub const BASE_ADDRESSES_1_03_0: BaseAddresses = BaseAddresses {
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
};

