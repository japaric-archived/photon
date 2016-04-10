//! Build Particle C dependencies

#![deny(warnings)]

extern crate tempdir;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use tempdir::TempDir;

macro_rules! try {
    ($e:expr) => {
        $e.unwrap_or_else(|e| panic!("{} with {}", stringify!($e), e))
    }
}

fn main() {
    let td = try!(TempDir::new("photon"));
    let td = &td.path();
    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    fetch(td);
    build(td);
    install(td, out_dir);
}

fn fetch(td: &Path) {
    const URL: &'static str = "https://github.com/spark/firmware";

    // XXX we should pin to some tag / commit
    // NOTE there isn't much to gain from using the git2 crate instead of just shelling out to
    // `git` because the build step will shell out to `make` and that `make` will also call the
    // `git` command
    assert!(try!(Command::new("git")
                     .args(&["clone", "--branch", "latest", "--depth", "1"])
                     .arg(URL)
                     .arg(td)
                     .status())
                .success());
}

fn build(td: &Path) {
    // XXX we probably don't need to build everything
    // NOTE(env_remove) Cargo sets the `TARGET` variable which messes up this `make` invocation
    // NOTE `-jN` seems to mess up the build so we omit it
    assert!(try!(Command::new("make")
                     .current_dir(td)
                     .env_remove("TARGET")
                     .env("PLATFORM", "photon")
                     .status())
                .success());
}

fn install(td: &Path, out_dir: &Path) {
    let src = &td.join("build/target");
    let dst = &out_dir.join("lib");

    rm_rf(dst);
    mkdir_p(dst);

    // XXX So many hard-coded paths...
    // Static libraries
    cp(src.join("user/platform-6-m/src/libuser.a"), dst);
    println!("cargo:rustc-link-lib=static=user");

    cp(src.join("hal-dynalib/platform-6-m/libhal-dynalib.a"), dst);
    println!("cargo:rustc-link-lib=static=hal-dynalib");

    cp(src.join("services-dynalib/arm/libservices-dynalib.a"), dst);
    println!("cargo:rustc-link-lib=static=services-dynalib");

    cp(src.join("system-dynalib/platform-6-m/libsystem-dynalib.a"),
       dst);
    println!("cargo:rustc-link-lib=static=system-dynalib");

    cp(src.join("rt-dynalib/platform-6-m/librt-dynalib.a"), dst);
    println!("cargo:rustc-link-lib=static=rt-dynalib");

    cp(src.join("wiring/platform-6-m/libwiring.a"), dst);
    println!("cargo:rustc-link-lib=static=wiring");

    cp(src.join("communication-dynalib/platform-6-m/libcommunication-dynalib.a"),
       dst);
    println!("cargo:rustc-link-lib=static=communication-dynalib");

    cp(src.join("platform/platform-6-m/libplatform.a"), dst);
    println!("cargo:rustc-link-lib=static=platform");

    cp(src.join("wiring_globals/platform-6-m/libwiring_globals.a"),
       dst);
    println!("cargo:rustc-link-lib=static=wiring_globals");

    println!("cargo:rustc-link-search={}", dst.display());

    // Object files
    // FIXME we shouldn't pollute the current directory BUT there is no search directory for object
    // files so we need pass them to the linker by relative/absolute path
    let src = td.join("build/target/user-part/platform-6-m/src");
    let dst = &env::current_dir().unwrap().join(".photon");

    rm_rf(dst);
    mkdir_p(dst);

    cp(src.join("user_module.o"), dst);

    cp(src.join("module_info.o"), dst);

    cp(src.join("user_export.o"), dst);

    cp(src.join("newlib_stubs.o"), dst);

    // Linker scripts
    // Some linker scripts use relative paths so we'll have build an elaborated directory structure
    // to keep them working
    let src = &td.join("modules/photon");
    let dst = &out_dir.join("ld");

    rm_rf(dst);
    mkdir_p(dst.join("shared/stm32f2xx"));
    mkdir_p(dst.join("photon/system-part1"));

    cp(src.join("user-part/linker.ld"), dst);

    cp(src.join("user-part/module_user_export.ld"), dst);

    cp(src.join("system-part1/module_system_part1_export.ld"),
       dst.join("photon/system-part1"));

    cp(src.join("system-part2/module_system_part2_export.ld"), dst);

    cp(td.join("build/arm/linker/stm32f2xx/backup_ram_memory.ld"),
       dst);

    cp(td.join("build/arm/linker/stm32f2xx/backup_ram_user.ld"),
       dst);

    cp(td.join("build/arm/linker/module_start.ld"), dst);

    cp(td.join("build/arm/linker/module_info.ld"), dst);

    cp(td.join("build/arm/linker/module_end.ld"), dst);

    cp(td.join("modules/shared/stm32f2xx/part1_vtor_module.ld"),
       dst.join("shared/stm32f2xx"));

    cp(td.join("modules/shared/stm32f2xx/user.ld"),
       dst.join("shared/stm32f2xx"));

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-search={}",
             dst.join("photon/system-part1").display());
}

fn cp<F, D>(file: F, dir: D)
    where F: AsRef<Path>,
          D: AsRef<Path>
{
    fn cp_(file: &Path, dir: &Path) {
        try!(fs::copy(file, dir.join(file.file_name().unwrap())));
    }

    cp_(file.as_ref(), dir.as_ref())
}

fn rm_rf<D>(dir: D)
    where D: AsRef<Path>
{
    fn rm_rf_(dir: &Path) {
        if dir.exists() {
            try!(fs::remove_dir_all(dir));
        }
    }

    rm_rf_(dir.as_ref())
}

fn mkdir_p<D>(dir: D)
    where D: AsRef<Path>
{
    try!(fs::create_dir_all(dir.as_ref()));
}
