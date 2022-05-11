use std::{env, fs, path::PathBuf, process::Command};

pub fn main() {
    build::rerun_if_changed("build.rs");

    let manifest_dir = build::cargo_manifest_dir();
    let impl_dir = manifest_dir.join("impl");
    let impl_wasm = manifest_dir.join("src/impl.wasm");

    // if packaged
    if !impl_dir.exists() {
        // if wasm module missing
        if !impl_wasm.exists() {
            // fail
            panic!("PACKAGING ERROR: this is a packaged build with the wasm module missing");
        } else {
            // do nothing
            return;
        }
    }

    // rebuild wasm module
    build::rerun_if_changed(&impl_dir);

    let cargo = build::cargo();
    let out_dir = build::out_dir();
    let impl_manifest = impl_dir.join("Cargo.toml");

    let cargo_home = None
        // override
        .or_else(|| Some(PathBuf::from(env::var_os("CARGO_HOME")?)))
        // default
        .or_else(|| {
            if cfg!(unix) {
                Some(PathBuf::from(env::var_os("HOME")?).join(".cargo"))
            } else if cfg!(windows) {
                Some(PathBuf::from(env::var_os("USERPROFILE")?).join(".cargo"))
            } else {
                None
            }
        })
        // we tried ¯\_(ツ)_/¯
        .unwrap_or_else(|| PathBuf::from("$CARGO_HOME"));

    let rustup_home = None
        // override
        .or_else(|| Some(PathBuf::from(env::var_os("RUSTUP_HOME")?)))
        // default
        .or_else(|| {
            if cfg!(unix) {
                Some(PathBuf::from("~/.rustup"))
            } else if cfg!(windows) {
                Some(PathBuf::from(env::var_os("USERPROFILE")?).join(".rustup"))
            } else {
                None
            }
        })
        // we tried ¯\_(ツ)_/¯
        .unwrap_or_else(|| PathBuf::from("$RUSTUP_HOME"));

    #[rustfmt::skip]
    let status = Command::new(cargo)
        .arg("build")
        .arg("--manifest-path").arg(&impl_manifest)
        .arg("--target-dir").arg(&out_dir)
        .args(["--target", "wasm32-unknown-unknown"])
        .arg("--release")
        .arg("--locked")
        .env("CARGO_ENCODED_RUSTFLAGS", format!(
            "\
                --remap-path-prefix={manifest_dir}=[cargo.manifest]\x1f\
                --remap-path-prefix={target_dir}=[cargo.target]\x1f\
                --remap-path-prefix={cargo_home}=[cargo.home]\x1f\
                --remap-path-prefix={rustup_home}=[rustup.home]\
            ",
            manifest_dir=manifest_dir.display(),
            target_dir=out_dir.display(),
            cargo_home=cargo_home.display(),
            rustup_home=rustup_home.display(),
        ))
        .status();

    // if sub compilation failed
    match status {
        Err(fail) => panic!("Compilation of wasm module failed: {fail}"),
        Ok(status) if status.success() => (),
        Ok(status) => panic!("Compilation of wasm module failed: {status}"),
    }

    // copy wasm into place
    let did_copy = fs::copy(
        out_dir.join("wasm32-unknown-unknown/release/semicoroutines_impl.wasm"),
        impl_wasm,
    );

    if let Err(fail) = did_copy {
        panic!("Failed to copy wasm module into place: {fail}");
    }
}
