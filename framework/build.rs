use std::env;
use std::fs::{copy, read_dir, create_dir_all, remove_dir_all};
use std::path::Path;
use std::process::Command;

fn main() {
    let out = env::var("PROFILE").unwrap();

    copy_dir_all("./js", format!("./target/{}/js", &out), true).unwrap();
    copy_dir_all("./node", format!("./target/{}/node", &out), false).unwrap();

    let success = Command::new("powershell")
        .args(["npm", "install"])
        .current_dir(format!("./target/{}/js", &out))
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success();

    if !success {
        panic!("Error building post install stuff");
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, ignore_node_modules: bool) -> std::io::Result<()> {
    remove_dir_all(&dst).unwrap_or(());
    create_dir_all(&dst)?;

    for entry in read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        
        if ignore_node_modules && entry.file_name() == "node_modules" {
        } else if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()), ignore_node_modules)?;
        } else {
            copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}