use std::fs;

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo::rustc-env=LIB=vcpkg_installed/x64-windows/lib");

    fs::copy(
        "vcpkg_installed/x64-windows/bin/SDL3.dll",
        "target/debug/SDL3.dll",
    )
    .unwrap();
    fs::copy(
        "vcpkg_installed/x64-windows/bin/SDL3_image.dll",
        "target/debug/SDL3_image.dll",
    )
    .unwrap();
    fs::copy(
        "vcpkg_installed/x64-windows/bin/SDL3_ttf.dll",
        "target/debug/SDL3_ttf.dll",
    )
    .unwrap();
}

#[cfg(target_os = "macos")]
fn main() {}
