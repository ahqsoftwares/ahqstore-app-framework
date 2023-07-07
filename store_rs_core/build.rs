fn main() {
    #[cfg(not(windows))]
    compile_error!("The AHQ Store APi can only run on windows based systems!");
}