use std::{
    env::{args, Args},
    fs,
    process::Command,
};

fn build(mut args: Args) {
    let is_release = args.any(|a| a == "--release");
    let profile = if is_release { "release" } else { "debug" };
    let arch = "x64";

    if is_release {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .expect("Could not spawn cargo");
    } else {
        Command::new("cargo")
            .arg("build")
            .status()
            .expect("Could not spawn cargo");
    }

    let out_dir = format!(
        "bin/{}-{}",
        if is_release { "release" } else { "debug" },
        arch
    );

    fs::create_dir_all(format!("{}/wharf", out_dir)).expect("couldn't create dir");
    fs::create_dir_all(format!("{}/sandbox", out_dir)).expect("couldn't create dir");

    // Copy engine
    fs::copy(
        format!("target/{}/libwharf.so", profile),
        format!("{}/wharf/libwharf.so", out_dir),
    )
    .expect("couldn't copy libwharf.so");

    // Copy sandbox
    fs::copy(
        format!("target/{}/sandbox", profile),
        format!("{}/sandbox/sandbox", out_dir),
    )
    .expect("couldn't copy sandbox");
    fs::copy(
        format!("target/{}/libwharf.so", profile),
        format!("{}/sandbox/libwharf.so", out_dir),
    )
    .expect("couldn't copy libwharf.so");
}

fn main() {
    build(args());
}
