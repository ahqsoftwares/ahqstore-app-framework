pub fn get_framework_exe(exe: &str) -> String {
    let mut splits = exe.split("\\").collect::<Vec<&str>>();

    splits.pop();
    splits.pop();
    splits.pop();

    splits.push("Framework");
    splits.push("framework.exe");

    splits.join("\\").to_string()
}
