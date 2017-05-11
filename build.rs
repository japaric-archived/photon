use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // linker scripts
    // NOTE some linker script have relative paths so we'll preserve the
    // directory layout of the particle/firmware repository.
    cp("modules/photon/user-part/linker.ld", out);
    cp("modules/photon/user-part/module_user_export.ld", out);
    cp(
        "modules/photon/system-part1/module_system_part1_export.ld",
        &out.join("photon/system-part1"),
    );
    cp(
        "modules/photon/system-part2/module_system_part2_export.ld",
        out,
    );
    cp("build/arm/linker/stm32f2xx/backup_ram_memory.ld", out);
    cp("build/arm/linker/stm32f2xx/backup_ram_user.ld", out);
    cp("build/arm/linker/module_start.ld", out);
    cp("build/arm/linker/module_info.ld", out);
    cp("build/arm/linker/module_end.ld", out);
    cp(
        "modules/shared/stm32f2xx/part1_vtor_module.ld",
        &out.join("shared/stm32f2xx"),
    );
    cp(
        "modules/shared/stm32f2xx/user.ld",
        &out.join("shared/stm32f2xx"),
    );

    // static libraries
    cp("build/target/user/platform-6-m/src/libuser.a", out);
    cp(
        "build/target/hal-dynalib/platform-6-m/libhal-dynalib.a",
        out,
    );
    cp(
        "build/target/services-dynalib/arm/libservices-dynalib.a",
        out,
    );
    cp(
        "build/target/system-dynalib/platform-6-m/libsystem-dynalib.a",
        out,
    );
    cp("build/target/rt-dynalib/platform-6-m/librt-dynalib.a", out);
    cp("build/target/wiring/platform-6-m/libwiring.a", out);
    cp(
        "build/target/communication-dynalib/platform-6-m/\
         libcommunication-dynalib.a",
        out,
    );
    cp("build/target/platform/platform-6-m/libplatform.a", out);
    cp(
        "build/target/wiring_globals/platform-6-m/libwiring_globals.a",
        out,
    );

    // library and linker script search paths
    println!("cargo:rustc-link-search={}", out.display());
    println!(
        "cargo:rustc-link-search={}",
        out.join("photon/system-part1").display()
    );
}

fn cp(src: &str, dst_dir: &Path) {
    fs::create_dir_all(dst_dir).ok();

    let filename = Path::new(src).file_name().unwrap();
    let src = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join(src);

    fs::copy(src, dst_dir.join(filename)).unwrap();
}
