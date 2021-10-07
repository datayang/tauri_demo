#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
// use std::process::{Command, Stdio};
use std::process::Command;
fn main() {
  // let cmd_str: String;
  // cmd_str = "start  D:\BeyondCompare4\BCompare.exe".to_string();
  // Command::new("cmd").arg("/c").arg(cmd_str).output().expect("cmd exec error!");

  let output = Command::new("C:\\WINDOWS\\system32\\notepad.exe")
    // .args(&["https://stackoverflow.com/"])
    // .output()
    .spawn()
    .expect("failed to execute process");

  let output2 = Command::new("C:\\WINDOWS\\system32\\notepad.exe")
    // .args(&["https://stackoverflow.com/"])
    // .stdin(Stdio::piped())
    // .stdout(Stdio::piped())
    // .output()
    .spawn()
    .expect("failed to execute process");

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
