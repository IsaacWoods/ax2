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
            qemu.arg("--no-reboot");
            qemu.arg("--no-shutdown");

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
    let loader = BuildComponent {
        artifact_name: "loader.efi",
        manifest_path: Path::new("kernel/loader/"),
        workspace: None,
        target: "x86_64-unknown-uefi",
        release: true,
        rustflags: None,
        features: vec![],
        build_std_components: false,
    }
    .build()?;
    std::fs::copy(loader, "build/efi/boot/bootx64.efi")?;

    let kernel = BuildComponent {
        artifact_name: "kernel",
        manifest_path: Path::new("kernel/"),
        workspace: Some(Path::new("kernel/")),
        target: "x86_64-unknown-none",
        release: true,
        rustflags: Some("-Crelocation-model=static"),
        features: vec![],
        build_std_components: true,
    }
    .build()?;
    std::fs::copy(kernel, "build/kernel.elf")?;

    Ok(())
}

#[derive(Debug)]
struct BuildComponent<'a, 'b, 'c, 'd, 'e> {
    artifact_name: &'a str,
    manifest_path: &'b Path,
    workspace: Option<&'c Path>,
    target: &'d str,
    release: bool,
    rustflags: Option<&'e str>,
    features: Vec<String>,
    build_std_components: bool,
}

impl BuildComponent<'_, '_, '_, '_, '_> {
    /// Build a component, returning the path at which the artifact can be found
    fn build(self) -> Result<PathBuf> {
        let mut cargo = Command::new("cargo");
        cargo.arg("build");

        cargo
            .arg("--manifest-path")
            .arg(self.manifest_path.join("Cargo.toml"));
        cargo.arg("--target").arg(self.target);
        if self.release {
            cargo.arg("--release");
        }
        if let Some(ref rustflags) = self.rustflags {
            cargo.env("RUSTFLAGS", rustflags);
        }
        if self.features.len() > 0 {
            cargo.arg("--features");
            cargo.arg(self.features.join(","));
        }
        if self.build_std_components {
            cargo.arg("-Zbuild-std=core,alloc");
            cargo.arg("-Zbuild-std-features=compiler-builtins-mem");
        }

        cargo
            .status()?
            .success()
            .then_some(())
            .ok_or(eyre!("Failed to build component: {:?}", self.manifest_path))?;

        // TODO: this will not work for things built with the host target
        let artifact_path = if let Some(workspace) = self.workspace {
            workspace
                .join("target")
                .join(self.target)
                .join(if self.release { "release" } else { "debug" })
                .join(self.artifact_name)
        } else {
            self.manifest_path
                .join("target")
                .join(self.target)
                .join(if self.release { "release" } else { "debug" })
                .join(self.artifact_name)
        };

        Ok(artifact_path)
    }
}
