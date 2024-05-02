mod codegen;

use std::ffi::OsStr;
use std::{env, fs, iter};

use anyhow::{bail, Context, Result};
use practice_tool_tasks::{cargo_command, project_root, steam_command, target_path, Distribution};

const APPID: u32 = 814380;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => dist()?,
        Some("codegen") => codegen::codegen(),
        Some("run") => run()?,
        Some("help") => print_help(),
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        r#"
Tasks:

run ........... compile and start the practice tool
dist .......... build distribution artifacts
codegen ....... generate Rust code: parameters, base addresses, ...
help .......... print this help
"#
    );
}

fn run() -> Result<()> {
    let status = cargo_command("build")
        .args(["--lib", "--package", "sekiro-practice-tool"])
        .status()
        .context("cargo")?;

    if !status.success() {
        bail!("cargo build failed");
    }

    fs::copy(
        project_root().join("jdsd_sekiro_practice_tool.toml"),
        target_path("debug").join("jdsd_sekiro_practice_tool.toml"),
    )?;

    let dll_path = target_path("debug").join("libjdsd_sekiro_practice_tool.dll").canonicalize()?;

    inject(iter::once(dll_path))?;

    Ok(())
}

fn dist() -> Result<()> {
    Distribution::new("jdsd_sekiro_practice_tool.zip")
        .with_artifact("libjdsd_sekiro_practice_tool.dll", "jdsd_sekiro_practice_tool.dll")
        .with_artifact("jdsd_sekiro_practice_tool.exe", "jdsd_sekiro_practice_tool.exe")
        .with_file("practice-tool/RELEASE-README.txt", "README.txt")
        .with_file("jdsd_sekiro_practice_tool.toml", "jdsd_sekiro_practice_tool.toml")
        .build(&["--locked", "--release", "--workspace", "--exclude", "xtask"])
}

fn inject<S: AsRef<OsStr>>(args: impl Iterator<Item = S>) -> Result<()> {
    cargo_command("build").args(["--release", "--bin", "inject"]).status().context("cargo")?;

    steam_command(target_path("release").join("inject"), APPID)?
        .args(args)
        .status()
        .context("inject")?;

    Ok(())
}
