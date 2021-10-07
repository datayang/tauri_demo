#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::WindowBuilder;

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let close = CustomMenuItem::new("close".to_string(), "关闭");
  let submenu = Submenu::new("文件", Menu::new().add_item(quit).add_item(close));
  let menu = Menu::new()
  .add_native_item(MenuItem::Copy)
  .add_item(CustomMenuItem::new("hide", "Hide"))
  .add_submenu(submenu);

  // let menu2 = Menu::new()
  // .add_item(CustomMenuItem::new("隐藏", "隐藏")); 
  
  tauri::Builder::default()
    //添加菜单
    .menu(menu)
    .on_menu_event(|event| {
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "close" => {
          event.window().close().unwrap();
        }
        _ => {}
      }
    })
    //创建一个窗口，添加菜单到特定窗口
    // .create_window(
    //   "main-window".to_string(),
    //   tauri::WindowUrl::App("home.html".into()),
    //   move |window_builder, webview_attributes| {
    //     (window_builder.menu(menu2), webview_attributes)
    //   },
    // )
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
