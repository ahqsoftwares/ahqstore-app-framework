fn main() {
    #[cfg(not(windows))]
    compile_error!("The AHQ Store APi can only run on windows based systems!");

    let profile = std::env::var("PROFILE");

    if let Ok(x) = profile {
        if &x == "release" && std::env::var("DOCS_RS").is_err() {}
    }
}
