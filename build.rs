#[cfg(target_os = "windows")]
fn main() {
    use std::{fs, path::Path};
    println!("cargo::rustc-env=LIB=vcpkg_installed/x64-windows/lib");

    for entry in Path::new("vcpkg_installed/x64-windows/bin")
        .read_dir()
        .unwrap()
    {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap() == "dll" {
            let to = format!(
                "target/debug/{}",
                path.file_name().unwrap().to_str().unwrap()
            );
            fs::copy(path, &to).unwrap();
        }
    }
}

#[cfg(target_os = "macos")]
fn main() {}
