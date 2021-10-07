#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate winreg;
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

use bindings::{
  Windows::Foundation::Uri,
  Windows::Web::Syndication::SyndicationClient,
  Windows::Win32::UI::WindowsAndMessaging::*,
};



fn main()  {
  //调用win 原生的一下东西  
  // unsafe {
  //   MessageBoxW(None, "这是提示","标题", MB_OK);
  // }

  test();
  test3();
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}

fn test3() -> io::Result<()> {
    println!("读取一些信息...");
    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hkcu.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion").unwrap();
    let pf: String = cur_ver.get_value("ProgramFilesDir").unwrap();
    let dp: String = cur_ver.get_value("DevicePath").unwrap();
    println!("ProgramFiles路径= {}\nDevicePath 路径= {}", pf, dp);

    for i in RegKey::predef(HKEY_LOCAL_MACHINE)
      .enum_keys().map(|x| x.unwrap())
      // .filter(|x| x.starts_with("."))
  {
      println!("MACHINE打印{}", i);
  }

    Ok(())
}


fn test() -> io::Result<()> {
  // println!("File extensions, registered in system:");
  for i in RegKey::predef(HKEY_CLASSES_ROOT)
      .enum_keys().map(|x| x.unwrap())
      .filter(|x| x.starts_with("."))
  {
      println!("root打印{}", i);
  }

  println!("开始打印user:");
  for i in RegKey::predef(HKEY_CURRENT_USER)
      .enum_keys().map(|x| x.unwrap())
      .filter(|x| x.starts_with("."))
  {
      println!("user打印{}", i);
  }


  Ok(())
}
