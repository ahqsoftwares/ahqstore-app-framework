//! # AHQ Store RS Core
//! The core crate for interfacing with the AHQ Store Framework
//! 
//! ## Example
//! ```
//! use ahq_store_rs_core::App;
//! fn main() {
//!     let app = App::start();
//! }
//! ```
//! 

mod utils;

#[cfg(not(debug_assertions))]
use std::env::current_exe;

use std::{
    os::windows::process::CommandExt,
    process::{Command, Stdio},
};

/// The Core App struct
pub struct App {
    _port: String,
    _password: String,
}

impl App {
    /// Starts your app using the AHQ Store Framework
    /// 
    /// ### Example
    /// ```
    /// use ahq_store_rs_core::App;
    /// 
    /// fn main() {
    ///     let app = App::start();
    /// }
    /// ```
    pub fn start() -> Self {
        #[cfg(not(debug_assertions))]
        let exe = current_exe().unwrap().to_str().unwrap();

        #[cfg(debug_assertions)]
        let exe: &str = &format!("{}\\ProgramData\\AHQ Store Applications\\Programs\\unknown_mock_app_idk_about\\app.exe", env!("SystemDrive"));

        let framework_path = utils::get_framework_exe(exe);

        if let Ok(child) = Command::new(framework_path)
            .creation_flags(0x08000000)
            .stdout(Stdio::piped())
            .spawn()
        {
            if let Ok(out) = child.wait_with_output() {
                if !out.status.success() {
                    panic!("AHQ Store Framework Failed");
                }

                if let Ok(x) = String::from_utf8(out.stdout) {
                    let splits = x.split(" ").collect::<Vec<&str>>();

                    let (port, pwd) = (splits[0], splits[1]);

                    if pwd.len() > 20 {
                        panic!("Password is too long");
                    }

                    return Self {
                        _port: port.into(),
                        _password: pwd.into(),
                    };
                } else {
                    panic!("Failed to parse data");
                }
            } else {
                panic!("Failed to get output from AHQ Store Framework");
            }
        } else {
            panic!("Failed to communicate with AHQ Store Framework");
        }
    }

    pub fn connect() {

    }
}
