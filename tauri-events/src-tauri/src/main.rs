#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::WindowBuilder;
use tauri::Manager;


#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main()  {
  
  tauri::Builder::default()
  //  .on_page_load(|window, _payload| {
  //   let label = window.label().to_string();
  //   window.listen("clicked".to_string(), move |_payload| {
  //     println!("got 'clicked' event on window '{}'", label);
  //   });
  // })
  .create_window(
    "Rust".to_string(),
    tauri::WindowUrl::App("index.html".into()),
    |window_builder, webview_attributes| {
      (window_builder.title("Tauri - Rust"), webview_attributes)
    },
  )
  .setup(|app| {
    let id = app.listen_global("clicked", |event| {
      println!("got event-name with payload {:?}", event.payload());
    });
    // app.unlisten(id);

    app.emit_all("datac", Payload { message: "Tauri is awesome!".into() }).unwrap();
    println!("执行完成");
    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}