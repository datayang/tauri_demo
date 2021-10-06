#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn my_custom_command() {
  println!("我是js操作的!");
}

#[tauri::command]
fn my_custom_command2(invoke_message: String) {
  println!("我是js操作的, 这是传的参数: {}", invoke_message);
}

#[tauri::command]
fn my_custom_command3() -> String {
  println!("返回数据的方法");
  "Hello from Rust!".into()
}

#[tauri::command]
async fn my_custom_command_w(window: tauri::Window) {
  println!("Window: {}", window.label());
}

//这个不太懂作用是什么
#[tauri::command]
async fn my_custom_command_app(app_handle: tauri::AppHandle) {
  let app_dir = app_handle.path_resolver().app_dir();
  use tauri::GlobalShortcutManager;
  app_handle.global_shortcut_manager().register("CTRL + U", move || {});
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command,my_custom_command2,my_custom_command3,my_custom_command_w,my_custom_command_app])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
