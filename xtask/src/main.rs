use eyre::{Result, eyre};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

xflags::xflags! {
    cmd app {
        cmd build {}
        cmd run {}
    }
}

fn main() -> Result<()> {
    let flags = App::from_env().unwrap();
    match flags.subcommand {
        AppCmd::Build(_build) => build()?,

        AppCmd::Run(_run) => {
            build()?;

            let mut qemu = Command::new("qemu-system-x86_64");
            qemu.arg("-enable-kvm");
            qemu.args(&["-machine", "q35"]);
            qemu.args(&["-cpu", "max,vmware-cpuid-freq,invtsc"]);
            qemu.args(&["-debugcon", "stdio"]);

            // Firmware
            qemu.args(&[
                "-drive",
                "if=pflash,format=raw,readonly=on,file=ovmf/code.fd",
            ]);
            qemu.args(&[
                "-drive",
                "if=pflash,format=raw,readonly=on,file=ovmf/vars.fd",
            ]);

            // Emulate `build` as a FAT filesystem
            qemu.args(&["-drive", "format=raw,file=fat:rw:build"]);

            qemu.status()?;
        }
    }

    Ok(())
}

fn build() -> Result<()> {
    let loader = build_component(
        "loader.efi",
        Path::new("kernel/loader/"),
        None,
        "x86_64-unknown-uefi",
        true,
    )?;
    std::fs::copy(loader, "build/efi/boot/bootx64.efi")?;

    Ok(())
}

/// Build a component, returning the path at which the artifact can be found
fn build_component(
    artifact_name: &str,
    manifest_path: &Path,
    workspace: Option<&Path>,
    target: &str,
    release: bool,
) -> Result<PathBuf> {
    let mut cargo = Command::new("cargo");
    cargo.arg("build");

    cargo
        .arg("--manifest-path")
        .arg(manifest_path.join("Cargo.toml"));
    cargo.arg("--target").arg(target);
    if release {
        cargo.arg("--release");
    }

    cargo
        .status()?
        .success()
        .then_some(())
        .ok_or(eyre!("Failed to build component: {:?}", manifest_path))?;

    // TODO: this will not work for things built with the host target
    let artifact_path = if let Some(workspace) = workspace {
        workspace
            .join("target")
            .join(target)
            .join(if release { "release" } else { "debug" })
            .join(artifact_name)
    } else {
        manifest_path
            .join("target")
            .join(target)
            .join(if release { "release" } else { "debug" })
            .join(artifact_name)
    };

    Ok(artifact_path)
}
