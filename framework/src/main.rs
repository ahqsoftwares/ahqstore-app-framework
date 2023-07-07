use std::{
    env::{args, current_exe},
    process::Command,
};

fn main() {
    let mut id = 0;
    let args = args()
        .filter(|_| {
            id += 1;
            id != 1
        })
        .collect::<Vec<String>>();

    let exe = current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace("\\framework.exe", "");

    if let Ok(mut x) = Command::new(format!("{}\\node\\node.exe", &exe))
        .arg("index.js")
        .args(args)
        .current_dir(format!("{}\\js", &exe))
        .spawn()
    {
        let _ = x.wait();
    }
}
