use std::env;
use std::path::{Path, PathBuf};

use practice_tool_tasks::codegen::{self, aob_direct, aob_indirect, aob_indirect_twice};
use textwrap::dedent;

fn patches_paths() -> impl Iterator<Item = PathBuf> {
    let base_path = PathBuf::from(
        env::var("SEKIRO_PATCHES_PATH").unwrap_or_else(|_| panic!("{}", dedent(r"
            SEKIRO_PATCHES_PATH environment variable undefined.
            Check the documentation: https://github.com/veeenu/darksoulsiii-practice-tool/README.md#building
        "))),
    );
    base_path
        .read_dir()
        .expect("Couldn't scan patches directory")
        .map(Result::unwrap)
        .map(|dir| dir.path().join("sekiro.exe"))
}

fn base_addresses_rs_path() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
        .join("lib")
        .join("libsekiro")
        .join("src")
        .join("codegen")
        .join("base_addresses.rs")
}

pub fn get_base_addresses() {
    let aobs = &[
        aob_indirect_twice(
            "Quitout",
            &["48 8B 05 ?? ?? ?? ?? 48 63 C9 89 54 88 20 C3"],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "RenderWorld",
            &["80 3D ?? ?? ?? ?? 00 0F 10 00 0F 11 45 D0"],
            2,
            7,
            true,
        ),
        aob_indirect_twice(
            "DebugRender",
            &["44 0F B6 3D ?? ?? ?? ?? 0F 29 74 24 20 0F 28 F1 E8"],
            4,
            8,
            true,
        ),
        aob_indirect_twice(
            "Igt",
            &[
                r#"48 8B 0D ?? ?? ?? ?? 0F 28 C6 F3 0F 59 05 ?? ?? ?? ?? F3 48 0F 2C C0 01 81 ?? ?? ?? ??"#,
            ],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "PlayerPosition",
            &["48 83 3D ?? ?? ?? ?? 00 0F 84 ?? ?? ?? ?? F3 41 0F 10 47 78 F3 0F 5C C7"],
            3,
            8,
            true,
        ),
        aob_indirect_twice(
            "DebugFlags",
            &["80 3D ?? ?? ?? ?? 00 75 08 32 C0 48 83 C4 20"],
            2,
            7,
            true,
        ),
        aob_indirect_twice(
            "ShowCursor",
            &[r#"40 38 3D ?? ?? ?? ?? 0F B6 DB 0F 44 DF 84 DB 0F 94 C3 83 7D 40 FF"#],
            3,
            7,
            true,
        ),
        aob_direct(
            "NoLogo",
            &[
                r#"74 30 48 8D 54 24 30 48 8B CD E8 ?? ?? ?? ?? 90 BB 01 00 00 00 89 5C 24 20 44 0F B6 4E 04"#,
            ],
            false,
        ),
        aob_direct(
            "FontPatch",
            &["48 8b fa 49 8b f0 48 8b d9"],
            false,
        ),
        aob_indirect_twice(
            "DebugShow",
            &["48 8B 05 ?? ?? ?? ?? 48 8B D9 48 85 C0 75 2E 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 4C 8B C8 4C 8D 05 ?? ?? ?? ?? BA ?? ?? ?? ?? 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ?? 48 8B 80 ?? ?? ?? ?? 48 85 C0 75 07 48 8B 83 ?? ?? ?? ?? 48 83 C4 20 5B C3"],
            3,
            7,
            true,
        ),
        aob_indirect_twice(
            "GrappleDebug",
            &["48 8B 05 ?? ?? ?? ?? 48 85 C0 75 2E 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 4C 8B C8 4C 8D 05 ?? ?? ?? ?? BA ?? ?? ?? ?? 48 8D 0D ?? ?? ?? ?? E8 ?? ?? ?? ?? 48 8B 05 ?? ?? ?? ?? 48 8B 80 ?? ?? ?? ?? 48 8D 54 24 50 48 8D 4C 24 40 48 8B 58 08 E8 ?? ?? ?? ?? 4C 8D 4C 24 40 C7 44 24 28 ?? ?? ?? ?? 4C 8D 84 24 ?? ?? ?? ?? F3 0F 11 74 24 20"],
            3,
            7,
            true,
        ),
    ];

    codegen::codegen_base_addresses(base_addresses_rs_path(), patches_paths(), aobs)
}
